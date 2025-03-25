Generates svg data by filling these areas or leaving them empty.

![Image showing the outlines of the areas](./example-icons/outlines.svg)

Every area has a probability of about 50% of being filled.

The library performs text to image hashing and if the crate
feature "rand" is enabled can also generate random symbols.

# Examples

- "Lorem ipsum dolor sit amet"

    ![the hashed image](./example-icons/Lorem%20ipsum.svg)

- "आइकन 🙂 ▒"

    ![the hashed image](./example-icons/non_ascii.svg)

- ""

    ![the hashed image of an empty string](./example-icons/empty.svg)

- Example for a random one

    ![a random icon](./example-icons/random.svg)