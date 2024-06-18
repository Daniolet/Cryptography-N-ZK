use ark_ff::PrimeField;
use polynomial::{ark_poly::domain, interface::UnivariantPolynomialInterface, univariant::UnivariantPolynomial, utils::compute_domain};

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub struct Witness<F: PrimeField> {
    /// The public input to the circuit
    pub public_input: Vec<F>,
    /// The auxiliary input to the circuit (private input)
    pub auxiliary_input: Vec<F>,
}

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub struct R1CS<F: PrimeField> {
    /// This is the C matrix
    pub c: Vec<Vec<F>>,
    /// This is the A matrix
    pub a: Vec<Vec<F>>,
    /// This is the B matrix
    pub b: Vec<Vec<F>>,
}

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub struct QAPPolysCoefficients<F: PrimeField> {
    pub a: Vec<Vec<F>>,
    pub b: Vec<Vec<F>>,
    pub c: Vec<Vec<F>>,
}

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub struct QAPPolys<F: PrimeField> {
    pub a: Vec<UnivariantPolynomial<F>>,
    pub b: Vec<UnivariantPolynomial<F>>,
    pub c: Vec<UnivariantPolynomial<F>>,
}

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub struct QAP<F: PrimeField> {
    /// This is the C matrix * witness in polynomial form
    pub cx: UnivariantPolynomial<F>,
    /// This is the A matrix * witness in polynomial form
    pub ax: UnivariantPolynomial<F>,
    /// This is the B matrix * witness in polynomial form
    pub bx: UnivariantPolynomial<F>,
    /// this is the t polynomial
    pub t: UnivariantPolynomial<F>,
    /// this is the h polynomial
    pub h: UnivariantPolynomial<F>,
}

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub struct ToxicWaste<F: PrimeField> {
    alpha: F,
    beta: F,
    gamma: F,
    delta: F,
    tau: F,
}

/// This is the trusted setup
/// handles;
/// Circuit specific trusted setup and noc-specific trusted setup
#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub struct TrustedSetup<F: PrimeField> {
    toxic_waste: ToxicWaste<F>,
    number_of_constraints: usize,
}

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub struct TrustedSetupExcecution<F: PrimeField> {
    powers_of_tau_g1: Vec<F>,
    powers_of_tau_g2: Vec<F>,
    alpha_g1: F,
    beta_g1: F,
    delta_g1: F,
    beta_g2: F,
    delta_g2: F,
}

impl<F: PrimeField> Witness<F> {
    pub fn new(public_input: Vec<F>, auxiliary_input: Vec<F>) -> Self {
        Self {
            public_input,
            auxiliary_input,
        }
    }

    pub fn render(&self) -> Vec<F> {
        let mut ren = self.public_input.clone();
        ren.extend(self.auxiliary_input.clone());
        ren
    }
}

impl<F: PrimeField> TrustedSetup<F> {
    pub fn new(&self, toxic_waste: ToxicWaste<F>, number_of_constraints: usize) -> Self {
        Self {
            toxic_waste,
            number_of_constraints,
        }
    }
}

impl<F: PrimeField> QAP<F> {
    pub fn new(
        cx: UnivariantPolynomial<F>,
        ax: UnivariantPolynomial<F>,
        bx: UnivariantPolynomial<F>,
        t: UnivariantPolynomial<F>,
        h: UnivariantPolynomial<F>,
    ) -> Self {
        Self { cx, ax, bx, t, h }
    }

    pub fn compute_ht(&self) -> UnivariantPolynomial<F> {
        self.h.clone() * self.t.clone()
    }

    pub fn qap_check(&self) -> bool {
        let ht = self.compute_ht();
        let lhs = self.ax.clone() * self.bx.clone();
        lhs == ht + self.cx.clone()
    }
}

impl<F: PrimeField> QAPPolysCoefficients<F> {
    pub fn new(a: Vec<Vec<F>>, b: Vec<Vec<F>>, c: Vec<Vec<F>>) -> Self {
        Self { a, b, c }
    }

    pub fn into_poly_rep(&self) -> QAPPolys<F> {
        let domain_lenght = self.a[0].len();
        let domain = compute_domain(domain_lenght);
        
        let a = self
            .a
            .iter()
            .map(|y| UnivariantPolynomial::interpolate(y.clone(), domain.clone()))
            .collect();
        let b = self
            .b
            .iter()
            .map(|y| UnivariantPolynomial::interpolate(y.clone(), domain.clone()))
            .collect();
        let c = self
            .c
            .iter()
            .map(|y| UnivariantPolynomial::interpolate(y.clone(), domain.clone()))
            .collect();

        QAPPolys { a, b, c }
    }
}
