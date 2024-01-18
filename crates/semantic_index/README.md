
# Semantic Index

## Evaluation

### Metrics

nDCG@k:
- "The value of NDCG is determined by comparing the relevance of the items returned by the search engine to the relevance of the item that a hypothetical "ideal" search engine would return.
- "The relevance of result is represented by a score (also known as a 'grade') that is assigned to the search query. The scores of these results are then discounted based on their position in the search results -- did they get recommended first or last?"

MRR@k:
- "Mean reciprocal rank quantifies the rank of the first relevant item found in the recommendation list."

MAP@k:
- "Mean average precision averages the precision@k metric at each relevant item position in the recommendation list.

Resources:
- [Evaluating recommendation metrics](https://www.shaped.ai/blog/evaluating-recommendation-systems-map-mmr-ndcg)
- [Math Walkthrough](https://towardsdatascience.com/demystifying-ndcg-bee3be58cfe0)
