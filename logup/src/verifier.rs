use arithmetic::multilinear_poly::{eq_eval, evaluate_on_point};
use ark_ec::pairing::Pairing;
use ark_std::One;
use merlin::Transcript;
use pcs::PolynomialCommitmentScheme;
use poly_iop::sum_check::SumCheck;
use utils::{append_serializable_element, get_and_append_challenge};

use crate::{Logup, LogupProof, VerifierParam, PCS};

// impl Logup {
//     pub fn verify<E: Pairing>(
//         a: &Vec<E::ScalarField>,
//         t: &Vec<E::ScalarField>,
//         commit_a: &Vec<<E as Pairing>::G1Affine>,
//         vk: &VerifierParam<E>,
//         proof: &LogupProof<E>,
//         transcript: &mut Transcript,
//     ) -> bool {
//         let m = a.len();
//         let n = t.len();

//         let mut affine_deque = proof.affine_deque.clone();
//         let mut field_deque = proof.field_deque.clone();

//         let r = get_and_append_challenge::<E::ScalarField>(transcript, b"Internal round");

//         let commit_f = affine_deque.pop_front().unwrap();
//         append_serializable_element(transcript, b"commitment", &commit_f);
//         let commit_g = affine_deque.pop_front().unwrap();
//         append_serializable_element(transcript, b"commitment", &commit_g);
//         let commit_e = affine_deque.pop_front().unwrap();
//         append_serializable_element(transcript, b"commitment", &commit_e);

//         // verify \sum_i f_i = \sum_i g_i * e_i
//         let sums = field_deque.pop_front().unwrap();
//         append_serializable_element(transcript, b"value", &sums);
//         assert_eq!(sums[0], sums[1]);

//         // sum-check on \sum_i f_i
//         let (challenge, value) =
//             SumCheck::verify(sums[0], 1, m.ilog2() as usize, transcript, &mut field_deque);
//         let proof = affine_deque.pop_front().unwrap();
//         append_serializable_element(transcript, b"open", &proof);
//         assert!(PCS::<E>::verify(vk, &commit_f, &challenge, &proof, value));

//         // sum-check on \sum_i g_i * e_i
//         let (challenge, value) =
//             SumCheck::verify(sums[1], 2, n.ilog2() as usize, transcript, &mut field_deque);
//         let open_value = field_deque.pop_front().unwrap();
//         assert_eq!(value, open_value[0] * open_value[1]);
//         let proof = affine_deque.pop_front().unwrap();
//         append_serializable_element(transcript, b"open", &proof);
//         assert!(PCS::<E>::verify(
//             vk,
//             &commit_g,
//             &challenge,
//             &proof,
//             open_value[0]
//         ));
//         let proof = affine_deque.pop_front().unwrap();
//         append_serializable_element(transcript, b"open", &proof);
//         assert!(PCS::<E>::verify(
//             vk,
//             &commit_e,
//             &challenge,
//             &proof,
//             open_value[1]
//         ));
//         append_serializable_element(transcript, b"value", &open_value);

//         // verify \sum_i eq(r', i) * f_i * (r + a_i) = 1
//         let point: Vec<_> = (0..m.ilog2() as usize)
//             .map(|_| get_and_append_challenge::<E::ScalarField>(transcript, b""))
//             .collect();
//         let (challenge, value) = SumCheck::verify(
//             E::ScalarField::one(),
//             3,
//             m.ilog2() as usize,
//             transcript,
//             &mut field_deque,
//         );
//         let open_value = field_deque.pop_front().unwrap();
//         assert_eq!(
//             value,
//             eq_eval(&point, &challenge) * open_value[0] * (open_value[1] + r)
//         );
//         let proof = affine_deque.pop_front().unwrap();
//         append_serializable_element(transcript, b"open", &proof);
//         assert!(PCS::<E>::verify(
//             vk,
//             &commit_f,
//             &challenge,
//             &proof,
//             open_value[0]
//         ));
//         let proof = affine_deque.pop_front().unwrap();
//         append_serializable_element(transcript, b"open", &proof);
//         assert!(PCS::<E>::verify(
//             vk,
//             &commit_a,
//             &challenge,
//             &proof,
//             open_value[1]
//         ));
//         append_serializable_element(transcript, b"value", &open_value);

//         // verfiy \sum_i eq(r', i) * g_i * (r + t_i) = 1
//         let point: Vec<_> = (0..n.ilog2() as usize)
//             .map(|_| get_and_append_challenge::<E::ScalarField>(transcript, b""))
//             .collect();
//         let (challenge, value) = SumCheck::verify(
//             E::ScalarField::one(),
//             3,
//             n.ilog2() as usize,
//             transcript,
//             &mut field_deque,
//         );
//         let proof = affine_deque.pop_front().unwrap();
//         append_serializable_element(transcript, b"open", &proof);
//         let open_value = field_deque.pop_front().unwrap()[0];
//         append_serializable_element(transcript, b"value", &open_value);
//         // assert_eq!(
//         //     value,
//         //     eq_eval(&point, &challenge) * open_value * (r + evaluate_on_point(t, &challenge)),
//         // );
//         if value
//             != eq_eval(&point, &challenge) * open_value * (r + evaluate_on_point(t, &challenge))
//         {
//             return false;
//         }
//         // assert!(PCS::<E>::verify(
//         //     vk, &commit_g, &challenge, &proof, open_value
//         // ));
//         if !PCS::<E>::verify(vk, &commit_g, &challenge, &proof, open_value) {
//             return false;
//         }

//         true
//     }
// }
impl Logup {
    pub fn verify<E: Pairing>(
        a: &Vec<E::ScalarField>,
        t: &Vec<E::ScalarField>,
        commit_a: &Vec<<E as Pairing>::G1Affine>,
        vk: &VerifierParam<E>,
        proof: &LogupProof<E>,
        transcript: &mut Transcript,
    ) -> bool {
        let m = a.len();
        let n = t.len();

        let mut affine_deque = proof.affine_deque.clone();
        let mut field_deque = proof.field_deque.clone();

        let r = get_and_append_challenge::<E::ScalarField>(transcript, b"Internal round");

        let commit_f = affine_deque.pop_front().unwrap();
        append_serializable_element(transcript, b"commitment", &commit_f);
        let commit_g = affine_deque.pop_front().unwrap();
        append_serializable_element(transcript, b"commitment", &commit_g);
        let commit_e = affine_deque.pop_front().unwrap();
        append_serializable_element(transcript, b"commitment", &commit_e);

        // verify \sum_i f_i = \sum_i g_i * e_i
        let sums = field_deque.pop_front().unwrap();
        append_serializable_element(transcript, b"value", &sums);
        if sums[0] != sums[1] {
            return false;
        }

        // sum-check on \sum_i f_i
        let (challenge, value) =
            SumCheck::verify(sums[0], 1, m.ilog2() as usize, transcript, &mut field_deque);
        let proof = affine_deque.pop_front().unwrap();
        append_serializable_element(transcript, b"open", &proof);
        if !PCS::<E>::verify(vk, &commit_f, &challenge, &proof, value) {
            return false;
        }

        // sum-check on \sum_i g_i * e_i
        let (challenge, value) =
            SumCheck::verify(sums[1], 2, n.ilog2() as usize, transcript, &mut field_deque);
        let open_value = field_deque.pop_front().unwrap();
        if value != open_value[0] * open_value[1] {
            return false;
        }
        let proof = affine_deque.pop_front().unwrap();
        append_serializable_element(transcript, b"open", &proof);
        if !PCS::<E>::verify(vk, &commit_g, &challenge, &proof, open_value[0]) {
            return false;
        }
        let proof = affine_deque.pop_front().unwrap();
        append_serializable_element(transcript, b"open", &proof);
        if !PCS::<E>::verify(vk, &commit_e, &challenge, &proof, open_value[1]) {
            return false;
        }
        append_serializable_element(transcript, b"value", &open_value);

        // verify \sum_i eq(r', i) * f_i * (r + a_i) = 1
        let point: Vec<_> = (0..m.ilog2() as usize)
            .map(|_| get_and_append_challenge::<E::ScalarField>(transcript, b""))
            .collect();
        let (challenge, value) = SumCheck::verify(
            E::ScalarField::one(),
            3,
            m.ilog2() as usize,
            transcript,
            &mut field_deque,
        );
        let open_value = field_deque.pop_front().unwrap();
        if value != eq_eval(&point, &challenge) * open_value[0] * (open_value[1] + r) {
            return false;
        }
        let proof = affine_deque.pop_front().unwrap();
        append_serializable_element(transcript, b"open", &proof);
        if !PCS::<E>::verify(vk, &commit_f, &challenge, &proof, open_value[0]) {
            return false;
        }
        let proof = affine_deque.pop_front().unwrap();
        append_serializable_element(transcript, b"open", &proof);
        if !PCS::<E>::verify(vk, &commit_a, &challenge, &proof, open_value[1]) {
            return false;
        }
        append_serializable_element(transcript, b"value", &open_value);

        // verfiy \sum_i eq(r', i) * g_i * (r + t_i) = 1
        let point: Vec<_> = (0..n.ilog2() as usize)
            .map(|_| get_and_append_challenge::<E::ScalarField>(transcript, b""))
            .collect();
        let (challenge, value) = SumCheck::verify(
            E::ScalarField::one(),
            3,
            n.ilog2() as usize,
            transcript,
            &mut field_deque,
        );
        let proof = affine_deque.pop_front().unwrap();
        append_serializable_element(transcript, b"open", &proof);
        let open_value = field_deque.pop_front().unwrap()[0];
        append_serializable_element(transcript, b"value", &open_value);
        if value
            != eq_eval(&point, &challenge) * open_value * (r + evaluate_on_point(t, &challenge))
        {
            return false;
        }
        if !PCS::<E>::verify(vk, &commit_g, &challenge, &proof, open_value) {
            return false;
        }

        true
    }
}
