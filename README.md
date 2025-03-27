Generates svg data by filling these areas or leaving them empty.

![an image showing the outlines of the areas](https://github.com/user-attachments/assets/39691ed2-576b-4b49-a36d-11f7c478f040)

Every area has a probability of about 50% of being filled.

The library performs text to image hashing and if the crate
feature "rand" is enabled can also generate random symbols.

# Examples

Note: Generated icons won't have a white background.

- "Lorem ipsum dolor sit amet"

    ![the hashed image](https://github.com/user-attachments/assets/c1920e2f-1b30-4960-9ab0-4e53b7c53e5e)

- "आइकन 🙂 ▒"

    ![the hashed image](https://github.com/user-attachments/assets/655e9c46-2726-4b14-9112-f15ab7072ea0)

- ""

    ![the hashed image of an empty string](https://github.com/user-attachments/assets/168a10d6-6ccb-4bd6-a685-d1080121fe57)

- Example for a random one

    ![random](https://github.com/user-attachments/assets/f2df4798-4004-4455-9fb5-86d4c4e687d9)
