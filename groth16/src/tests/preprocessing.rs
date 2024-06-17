use crate::{
    interfaces::R1CSProcessingInterface,
    preprocessing::PreProcessor,
    primitives::{QAPPolysCoefficients, Witness, R1CS},
};
use ark_test_curves::bls12_381::Fr;

#[test]
fn test_to_qap_poly_coefficients() {
    let r1cs = R1CS::<Fr> {
        a: vec![
            vec![Fr::from(2u32), Fr::from(1u32)],
            vec![Fr::from(2u32), Fr::from(5u32)],
            vec![Fr::from(2u32), Fr::from(5u32)],
            vec![Fr::from(2u32), Fr::from(5u32)],
        ],
        b: vec![
            vec![Fr::from(2u32), Fr::from(2u32)],
            vec![Fr::from(2u32), Fr::from(2u32)],
            vec![Fr::from(2u32), Fr::from(2u32)],
            vec![Fr::from(2u32), Fr::from(2u32)],
        ],
        c: vec![
            vec![Fr::from(2u32), Fr::from(2u32)],
            vec![Fr::from(2u32), Fr::from(2u32)],
            vec![Fr::from(2u32), Fr::from(2u32)],
            vec![Fr::from(2u32), Fr::from(2u32)],
        ],
    };

    let qap_poly_coefficients = r1cs.to_qap_poly_coefficients();

    let excpected_result = QAPPolysCoefficients {
        a: vec![
            vec![
                Fr::from(2u32),
                Fr::from(2u32),
                Fr::from(2u32),
                Fr::from(2u32),
            ],
            vec![
                Fr::from(1u32),
                Fr::from(5u32),
                Fr::from(5u32),
                Fr::from(5u32),
            ],
        ],
        b: vec![
            vec![
                Fr::from(2u32),
                Fr::from(2u32),
                Fr::from(2u32),
                Fr::from(2u32),
            ],
            vec![
                Fr::from(2u32),
                Fr::from(2u32),
                Fr::from(2u32),
                Fr::from(2u32),
            ],
        ],
        c: vec![
            vec![
                Fr::from(2u32),
                Fr::from(2u32),
                Fr::from(2u32),
                Fr::from(2u32),
            ],
            vec![
                Fr::from(2u32),
                Fr::from(2u32),
                Fr::from(2u32),
                Fr::from(2u32),
            ],
        ],
    };

    assert_eq!(qap_poly_coefficients.a, excpected_result.a);
    assert_eq!(qap_poly_coefficients.b, excpected_result.b);
    assert_eq!(qap_poly_coefficients.c, excpected_result.c);
}


#[test]
fn test_to_qap_poly_coefficients_0() {
    // [0, 1, 0, 0, 0],
    // [0, 0, 1, 0, 0],
    // [0, 0, 0, 1, 0],
    // [0, 0, 0, 0, 1],
    // [0, 1, 0, 0, 0],
    // [0, 1, 0, 0, 0],
    // [0, 0, 1, 0, 0],
    let r1cs = R1CS::<Fr> {
        a: vec![
            vec![Fr::from(0u32), Fr::from(1u32), Fr::from(0u32), Fr::from(0u32), Fr::from(0u32)],
            vec![Fr::from(0u32), Fr::from(0u32), Fr::from(1u32), Fr::from(0u32), Fr::from(0u32)],
            vec![Fr::from(0u32), Fr::from(0u32), Fr::from(0u32), Fr::from(1u32), Fr::from(0u32)],
            vec![Fr::from(0u32), Fr::from(0u32), Fr::from(0u32), Fr::from(0u32), Fr::from(1u32)],
            vec![Fr::from(0u32), Fr::from(1u32), Fr::from(0u32), Fr::from(0u32), Fr::from(0u32)],
            vec![Fr::from(0u32), Fr::from(1u32), Fr::from(0u32), Fr::from(0u32), Fr::from(0u32)],
            vec![Fr::from(0u32), Fr::from(0u32), Fr::from(1u32), Fr::from(0u32), Fr::from(0u32)],
        ],
        b: vec![
            vec![Fr::from(0u32), Fr::from(1u32), Fr::from(0u32), Fr::from(0u32), Fr::from(0u32)],
            vec![Fr::from(0u32), Fr::from(0u32), Fr::from(1u32), Fr::from(0u32), Fr::from(0u32)],
            vec![Fr::from(0u32), Fr::from(0u32), Fr::from(0u32), Fr::from(1u32), Fr::from(0u32)],
            vec![Fr::from(0u32), Fr::from(0u32), Fr::from(0u32), Fr::from(0u32), Fr::from(1u32)],
            vec![Fr::from(0u32), Fr::from(1u32), Fr::from(0u32), Fr::from(0u32), Fr::from(0u32)],
            vec![Fr::from(0u32), Fr::from(1u32), Fr::from(0u32), Fr::from(0u32), Fr::from(0u32)],
            vec![Fr::from(0u32), Fr::from(0u32), Fr::from(1u32), Fr::from(0u32), Fr::from(0u32)],
        ],
        c: vec![
            vec![Fr::from(0u32), Fr::from(1u32), Fr::from(0u32), Fr::from(0u32), Fr::from(0u32)],
            vec![Fr::from(0u32), Fr::from(0u32), Fr::from(1u32), Fr::from(0u32), Fr::from(0u32)],
            vec![Fr::from(0u32), Fr::from(0u32), Fr::from(0u32), Fr::from(1u32), Fr::from(0u32)],
            vec![Fr::from(0u32), Fr::from(0u32), Fr::from(0u32), Fr::from(0u32), Fr::from(1u32)],
            vec![Fr::from(0u32), Fr::from(1u32), Fr::from(0u32), Fr::from(0u32), Fr::from(0u32)],
            vec![Fr::from(0u32), Fr::from(1u32), Fr::from(0u32), Fr::from(0u32), Fr::from(0u32)],
            vec![Fr::from(0u32), Fr::from(0u32), Fr::from(1u32), Fr::from(0u32), Fr::from(0u32)],
        ],
    };
    
    let qap_poly_coefficients = r1cs.to_qap_poly_coefficients();
    let excpected_result = QAPPolysCoefficients {
        a: vec![
            vec![
                Fr::from(0u32),
                Fr::from(0u32),
                Fr::from(0u32),
                Fr::from(0u32),
                Fr::from(0u32),
                Fr::from(0u32),
                Fr::from(0u32),
            ],
            vec![
                Fr::from(1u32),
                Fr::from(0u32),
                Fr::from(0u32),
                Fr::from(0u32),
                Fr::from(1u32),
                Fr::from(1u32),
                Fr::from(0u32),
            ],
            vec![
                Fr::from(0u32),
                Fr::from(1u32),
                Fr::from(0u32),
                Fr::from(0u32),
                Fr::from(0u32),
                Fr::from(0u32),
                Fr::from(1u32),
            ],
            vec![
                Fr::from(0u32),
                Fr::from(0u32),
                Fr::from(1u32),
                Fr::from(0u32),
                Fr::from(0u32),
                Fr::from(0u32),
                Fr::from(0u32),
            ],
            vec![
                Fr::from(0u32),
                Fr::from(0u32),
                Fr::from(0u32),
                Fr::from(1u32),
                Fr::from(0u32),
                Fr::from(0u32),
                Fr::from(0u32),
            ],
        ],
        b: vec![
            vec![
                Fr::from(0u32),
                Fr::from(0u32),
                Fr::from(0u32),
                Fr::from(0u32),
                Fr::from(0u32),
                Fr::from(0u32),
                Fr::from(0u32),
            ],
            vec![
                Fr::from(1u32),
                Fr::from(0u32),
                Fr::from(0u32),
                Fr::from(0u32),
                Fr::from(1u32),
                Fr::from(1u32),
                Fr::from(0u32),
            ],
            vec![
                Fr::from(0u32),
                Fr::from(1u32),
                Fr::from(0u32),
                Fr::from(0u32),
                Fr::from(0u32),
                Fr::from(0u32),
                Fr::from(1u32),
            ],
            vec![
                Fr::from(0u32),
                Fr::from(0u32),
                Fr::from(1u32),
                Fr::from(0u32),
                Fr::from(0u32),
                Fr::from(0u32),
                Fr::from(0u32),
            ],
            vec![
                Fr::from(0u32),
                Fr::from(0u32),
                Fr::from(0u32),
                Fr::from(1u32),
                Fr::from(0u32),
                Fr::from(0u32),
                Fr::from(0u32),
            ],
        ],
        c: vec![
            vec![
                Fr::from(0u32),
                Fr::from(0u32),
                Fr::from(0u32),
                Fr::from(0u32),
                Fr::from(0u32),
                Fr::from(0u32),
                Fr::from(0u32),
            ],
            vec![
                Fr::from(1u32),
                Fr::from(0u32),
                Fr::from(0u32),
                Fr::from(0u32),
                Fr::from(1u32),
                Fr::from(1u32),
                Fr::from(0u32),
            ],
            vec![
                Fr::from(0u32),
                Fr::from(1u32),
                Fr::from(0u32),
                Fr::from(0u32),
                Fr::from(0u32),
                Fr::from(0u32),
                Fr::from(1u32),
            ],
            vec![
                Fr::from(0u32),
                Fr::from(0u32),
                Fr::from(1u32),
                Fr::from(0u32),
                Fr::from(0u32),
                Fr::from(0u32),
                Fr::from(0u32),
            ],
            vec![
                Fr::from(0u32),
                Fr::from(0u32),
                Fr::from(0u32),
                Fr::from(1u32),
                Fr::from(0u32),
                Fr::from(0u32),
                Fr::from(0u32),
            ],
        ],
    };
    
    assert_eq!(qap_poly_coefficients, excpected_result);
}
