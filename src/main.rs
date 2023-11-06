use lambdaworks_math::field::fields::u64_prime_field::U64PrimeField;
use lambdaworks_math::field::element::FieldElement;

use crate::rescue::Rescue;
mod rescue;


fn main() {
    const PRIME: u64 = 2147483647;
    let rescue = Rescue::new(12, 6, 8);
    
    let input_sequence: Vec<FieldElement<U64PrimeField<PRIME>>> = vec![
        FieldElement::<U64PrimeField<PRIME>>::from(0),
        FieldElement::<U64PrimeField<PRIME>>::from(1),
        FieldElement::<U64PrimeField<PRIME>>::from(2),
        FieldElement::<U64PrimeField<PRIME>>::from(3),
        FieldElement::<U64PrimeField<PRIME>>::from(4),
        FieldElement::<U64PrimeField<PRIME>>::from(5),
        FieldElement::<U64PrimeField<PRIME>>::from(6),
        FieldElement::<U64PrimeField<PRIME>>::from(7),
        FieldElement::<U64PrimeField<PRIME>>::from(8),
        FieldElement::<U64PrimeField<PRIME>>::from(9),
        FieldElement::<U64PrimeField<PRIME>>::from(10),
        FieldElement::<U64PrimeField<PRIME>>::from(11),
    ];
    let hash = rescue.hash(input_sequence);
    
    println!("{:?}", hash);
}

