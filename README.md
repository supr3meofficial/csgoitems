# CSGOItems
A CSGO item data library for my [CSGOStash Scraper](https://github.com/supr3meofficial/csgostash-scraper).

Current version: [v0.1](https://github.com/supr3meofficial/csgoitems/releases/tag/v0.1)

## Data Model Example:

```rust
ItemSkin {
            name: "★ Driver Gloves | Convoy",
            item_type: Glove,
            rarity: Covert,
            description: "These driving gloves offer protection from the elements while still maintaining tactile sensation. It has been made with a mix of brown soft leather and dyed suede.",
            lore: "Sometimes discretion is called for",
            can_be_souvenir: false,
            can_be_stattrak: false,
            wears: [
                BattleScarred(
                    "https://steamcommunity-a.akamaihd.net/economy/image/-9a81dlWLwJ2UUGcVs_nsVtzdOEdtWwKGZZLQHTxDZ7I56KU0Zwwo4NUX4oFJZEHLbXH5ApeO4YmlhxYQknCRvCo04DAX1R3LjtQurWzLhRfwP_BcjZ94dW6nZSKhe7LO77QgHJu5MRjjeyPpI2ni1Cw-BZsMWjzJoOcdAc6M1-DrFTslbzrhcXpuJqbynVjuXMg4mGdwULUCJdHpw/512fx384f",
                ),
                FactoryNew(
                    "https://steamcommunity-a.akamaihd.net/economy/image/-9a81dlWLwJ2UUGcVs_nsVtzdOEdtWwKGZZLQHTxDZ7I56KU0Zwwo4NUX4oFJZEHLbXH5ApeO4YmlhxYQknCRvCo04DAX1R3LjtQurWzLhRfwP_BcjZ94dW6nZSKhe7LP7LWnn9u5MRjjeyPrYj03lfl8ks-YDuldo7EdQZoZ1uCqAW3yL_vh8XptMuayXsws3Er4GGdwULV1ZH2Ag/512fx384f",
                ),
                FieldTested(
                    "https://steamcommunity-a.akamaihd.net/economy/image/-9a81dlWLwJ2UUGcVs_nsVtzdOEdtWwKGZZLQHTxDZ7I56KU0Zwwo4NUX4oFJZEHLbXH5ApeO4YmlhxYQknCRvCo04DAX1R3LjtQurWzLhRfwP_BcjZ94dW6nZSKhe7LPr7Vn35c18lwmO7Eu92s2FW1-ko4NWjxJYGdegE-YA3U-wC_lbvmgMe_tcidzXdquikntH3D30vgtGG3lFU/512fx384f",
                ),
                MinimalWear(
                    "https://steamcommunity-a.akamaihd.net/economy/image/-9a81dlWLwJ2UUGcVs_nsVtzdOEdtWwKGZZLQHTxDZ7I56KU0Zwwo4NUX4oFJZEHLbXH5ApeO4YmlhxYQknCRvCo04DAX1R3LjtQurWzLhRfwP_BcjZ94dW6nZSKhe7LP7LWnn9u5MRjjeyPrYj03lfl8ks-YDuldo7EdQZoZ1uCqAW3yL_vh8XptMuayXsws3Er4GGdwULV1ZH2Ag/512fx384f",
                ),
                WellWorn(
                    "https://steamcommunity-a.akamaihd.net/economy/image/-9a81dlWLwJ2UUGcVs_nsVtzdOEdtWwKGZZLQHTxDZ7I56KU0Zwwo4NUX4oFJZEHLbXH5ApeO4YmlhxYQknCRvCo04DAX1R3LjtQurWzLhRfwP_BcjZ94dW6nZSKhe7LPr7Vn35c18lwmO7Eu92s2FW1-ko4NWjxJYGdegE-YA3U-wC_lbvmgMe_tcidzXdquikntH3D30vgtGG3lFU/512fx384f",
                ),
            ],
        }
```