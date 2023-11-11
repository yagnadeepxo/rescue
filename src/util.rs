
use lambdaworks_math::field::element::FieldElement;
use lambdaworks_math::field::fields::u64_prime_field::U64PrimeField;

const PRIME: u64 = 2147483647;


pub fn linear_combination_u64(u: &[u64], v: &[FieldElement<U64PrimeField<PRIME>>]) -> FieldElement<U64PrimeField<PRIME>> {
    assert_eq!(u.len(), v.len(), "The lengths of u and v must be the same.");

    let mut result = FieldElement::<U64PrimeField<PRIME>>::zero();
    
    for (ui, vi) in u.iter().zip(v.iter()) {
        // Perform the field multiplication and addition
        result = result + FieldElement::<U64PrimeField<PRIME>>::from(*ui) * vi;
    }
    
    result
}

const MATRIX_CIRC_MDS_12_SML: [u64; 12] = [9, 7, 4, 1, 16, 2, 256, 128, 3, 32, 1, 1];

// This function applies the circulant MDS matrix to the input state.
pub fn apply_circulant_12_sml(state: &mut [FieldElement<U64PrimeField<PRIME>>]) {
    // Check that the state has the correct length to apply the MDS matrix.
    assert_eq!(state.len(), 12, "State must be of length 12");

    let mut new_state = [FieldElement::<U64PrimeField<PRIME>>::zero(); 12];

    for i in 0..12 {
        // Generate the i-th row of the circulant matrix by rotating the first row
        let rotated_matrix_row = rotate_right(MATRIX_CIRC_MDS_12_SML, i);

        // Compute the linear combination of the state with the i-th row of the MDS matrix
        new_state[i] = linear_combination_u64(
            &rotated_matrix_row,
            state,
        );
    }

    for (s, &new_s) in state.iter_mut().zip(new_state.iter()) {
        *s = new_s;
    }
}

    // Helper function to rotate an array to the right.
    pub fn rotate_right<const N: usize>(input: [u64; N], offset: usize) -> [u64; N] {
        let mut output = [0u64; N];
        let offset = offset % N; // Ensure the offset is within the bounds of the array size
        for (i, item) in input.iter().enumerate() {
            output[(i + offset) % N] = *item;
        }
        output
    }


    pub fn add_round_constants(state: &mut [FieldElement<U64PrimeField<PRIME>>], round_constants: &[FieldElement<U64PrimeField<PRIME>>]) {
        for (s, rc) in state.iter_mut().zip(round_constants.iter()) {
            *s = *s + *rc; 
        }
    }

    pub fn sbox(state: &mut [FieldElement<U64PrimeField<PRIME>>], alpha: u64) {
        for i in 0..12 {
            state[i] = state[i].pow(alpha);  
        }
    }

    pub fn sbox_inv(state: &mut [FieldElement<U64PrimeField<PRIME>>], alpha_inv: u64) {
        for i in 0..12 {
            state[i] = state[i].pow(alpha_inv);  
        }
    }