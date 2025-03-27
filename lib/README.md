Generates svg data by filling these areas or leaving them empty.

<img src="example-icons/outlines.svg" alt="Image showing the outlines of the areas">

Every area has a probability of about 50% of being filled.

The library performs text to image hashing and if the crate
feature "rand" is enabled can also generate random symbols.

# Examples

Note: Generated icons won't have a white background.

- "Lorem ipsum dolor sit amet"

    ![the hashed image](lib/example-icons/Lorem%20ipsum.svg)

- "आइकन 🙂 ▒"

    ![the hashed image](lib/example-icons/non_ascii.svg)

- ""

    ![the hashed image of an empty string](lib/example-icons/empty.svg)

- Example for a random one

    ![a random icon](lib/example-icons/random.svg)