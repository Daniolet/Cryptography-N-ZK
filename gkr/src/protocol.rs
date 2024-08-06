use crate::{interfaces::GKRProtocolInterface, primitives::GKRProof, utils::gen_w_mle};
use ark_ff::PrimeField;
use circuits::{
    interfaces::GKRProtocolCircuitInterface,
    primitives::{Circuit, CircuitEvaluation},
};
use fiat_shamir::{interface::TranscriptInterface, FiatShamirTranscript};
use polynomial::{
    composed::multilinear::ComposedMultilinear, interface::MultilinearPolynomialInterface,
};
use sum_check::{
    composed::multicomposed::MultiComposedProver, interface::MultiComposedProverInterface,
};

pub struct GKRProtocol;

impl<F: PrimeField> GKRProtocolInterface<F> for GKRProtocol {
    fn prove(
        circuit: &Circuit,
        evals: &CircuitEvaluation<F>,
    ) -> GKRProof<F> {
        let mut transcript = FiatShamirTranscript::new(vec![]);
        let mut sum_check_proofs = vec![];
        let mut w_i_b = vec![];
        let mut w_i_c = vec![];

        let w_0_mle = gen_w_mle(&evals.layers, 0);
        transcript.append(w_0_mle.to_bytes());

        let mut n_r = transcript.sample_n_as_field_elements(w_0_mle.num_vars);
        let claim = w_0_mle.evaluate(&n_r).unwrap();
        

        // starting the GKR round reductions powered by sumcheck
        for l_index in 1..evals.layers.len() {
            let n_r_internal = n_r.clone();
            let number_of_round = n_r_internal.len();

            let (add_mle, mul_mle) = circuit.get_add_n_mul_mle::<F>(l_index - 1);
            let w_i_mle = gen_w_mle(&evals.layers, l_index);
            // f(b, c) = add(r, b, c)(w_i(b) + w_i(c)) + mul(r, b, c)(w_i(b) * w_i(c))
            // add(r, b, c) ---> add(b, c)
            let add_b_c =
                add_mle.partial_evaluations(n_r_internal.clone(), vec![0; number_of_round]);
            // mul(r, b, c) ---> mul(b, c)
            let mul_b_c = mul_mle.partial_evaluations(n_r_internal, vec![0; number_of_round]);

            let wb = w_i_mle.clone();
            let wc = w_i_mle.clone();

            // w_i(b) + w_i(c)
            let wb_add_wc = wb.add_distinct(&wc);
            // w_i(b) * w_i(c)
            let wb_mul_wc = wb.mul_distinct(&wc);

            println!("layer index: {}", l_index);
            println!("add_b_c: {}", add_b_c.num_vars());
            println!("add_b_c: {}", mul_b_c.num_vars());
            println!("add_mle: {}", add_mle.num_vars());
            println!("mul_mle: {}", mul_mle.num_vars());
            println!("wb: {}", wb.num_vars());
            println!("wb - raw: {:?}", evals.layers[l_index]);
            println!("wc: {}", wc.num_vars());
            println!("wb_add_wc: {}", wb_add_wc.num_vars());
            println!("wb_mul_wc: {}", wb_mul_wc.num_vars());
            
            //  add(b, c)(w_i(b) + w_i(c))
            let f_b_c_add_section = ComposedMultilinear::new(vec![add_b_c, wb_add_wc]);
            // mul(b, c)(w_i(b) * w_i(c))
            let f_b_c_mul_section = ComposedMultilinear::new(vec![mul_b_c, wb_mul_wc]);

            // f(b, c) = add(r, b, c)(w_i(b) + w_i(c)) + mul(r, b, c)(w_i(b) * w_i(c))
            let f_b_c = vec![f_b_c_add_section, f_b_c_mul_section];

            // this prover that the `claim` is the result of the evalution of the preivous layer
            let (sumcheck_proof, random_challenges) =
                MultiComposedProver::sum_check_proof_without_initial_polynomial(
                    &f_b_c,
                    &mut transcript,
                    &claim,
                );

            transcript.append(sumcheck_proof.to_bytes());
            sum_check_proofs.push(sumcheck_proof);

            let (rand_b, rand_c) = random_challenges.split_at(random_challenges.len() / 2);

            let eval_w_i_b = wb.evaluate(&rand_b.to_vec()).unwrap();
            let eval_w_i_c = wc.evaluate(&rand_c.to_vec()).unwrap();

            w_i_b.push(eval_w_i_b);
            w_i_c.push(eval_w_i_c);

            // TODO: perform mathematical proof bindings for the eval_w_i_b and eval_w_i_c
        }

        GKRProof {
            sum_check_proofs,
            w_i_b,
            w_i_c,
        }
    }

    fn verify(
        circuit: &Circuit,
        input: &[F],
        proof: &GKRProof<F>,
    ) -> bool {
        
        true
    }
}




#[cfg(test)]
mod tests {
    use circuits::{primitives::{CircuitLayer, Gate, GateType}, interfaces::CircuitInterface};
    use super::*;
    use ark_test_curves::bls12_381::Fr;
    

    
    
    // sample circuit evaluation
    //      100(*)    - layer 0
    //     /     \
    //   5(+)_0   20(*)_1 - layer 1
    //   / \    /  \
    //  2   3   4   5
    #[test]
    fn test_gkr_protocol() {
        let layer_0 = CircuitLayer::new(vec![Gate::new(GateType::Mul, [0, 1])]);
        let layer_1 = CircuitLayer::new(vec![
            Gate::new(GateType::Add, [0, 1]),
            Gate::new(GateType::Mul, [2, 3]),
        ]);
        let circuit = Circuit::new(vec![layer_0, layer_1]);
        let input = [
            Fr::from(2u32),
            Fr::from(3u32),
            Fr::from(4u32),
            Fr::from(5u32),
        ];
        let evaluation = circuit.evaluate(&input);
        
        
        let proof = GKRProtocol::prove(&circuit, &evaluation);
    }
}