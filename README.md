# Aitiquette

Aitiquette is a Rust app for auto-labelling links in [Linkwarden](https://linkwarden.app/) using any [OpenAI](https://openai.com/) compatible APIs (works with [Ollama](https://github.com/ollama/ollama)).

> The noun *aitiquette* is the combination of *ai* (artificial intelligence) and *étiquette* (the french word for *label*).

*THIS IS CURRENTLY A WORK IN PROGRESS...*

## Implementation

To update tags for a link, here is the minimal request body:

```json
{
    "collection": {
        "id": 1,
        "ownerId": 1
    },
    "tags": [
        {
            "name": "test"
        }
    ]
}
```

## Next steps

This is a list of features I would like to implement:

- [ ] Add an option to perform tagging on non-tagged links only or all.
- [X] Allow to select the language for tags.
- [X] Add support for more AI models (e.g. GPT-3, BLOOM, etc.)
- [ ] Customize prompts.
- [ ] Temperature control for prompts.
- [ ] Do not load and store all links at once for large instances. Study Iterators.
