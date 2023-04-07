# Arith 

- Daniel Diaz & Cameron Castillo 

For this assignment, TA Isaac Chen and Vincent Zhaung provided us with immense help in
conceptualizing Bitpack. Thanks to their assistance, we were able to successfully implement 
both the Compressor and Decompressor.

To ensure the correctness of our formulas, we modeled the forward and 
backward operations within the same .rs f ile, allowing us to perform 
round-trip testing.

- Architecture
  -
    - rpeg 
        - rbg.rs conversions formulas for rbg data. 
        - component_video_and_blocks.rs handle component video and blocks 
        conversions formulas.
        - dct_coeff.rs handle dct conversions formulas.
        - conversions.rs handles translate one format to another.
        - codec.rs handle how the transformations take place.
        - structs.rs holds all the struct used throughout the rpeg crate 
        - main.rs
    - bitpack
      - lib.rs 
      - bitpack.rs handle the bit shifting operations
    - Array2
      - lib.rs implementation model of the Array2 crate
        - trim function added directly to Array2 


We spent approximately a week brainstorming a solution for arith and striving to comprehensively understand the 
necessary steps. It was our belief that grasping the nuances of this assignment was four times 
more challenging than completing the actual task at hand.

Once we had a complete understanding of how to tackle each step, we were able to implement arith in approximately 
two days.
