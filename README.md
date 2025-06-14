Vault program using Pinocchio - The Zero Dependent Lib
 Once the concept of the discriminator is clear, everything else falls into place.
First, we match the instruction using the discriminator. Then, we deserialize the incoming byte data into their respective structs for both the instruction data and the accounts.
After deserialization, we validate each account and instruction field to ensure everything is as expected. Once all validations pass, we move on to the core logic of the instruction, such as handling a deposit or interacting with a vault.
