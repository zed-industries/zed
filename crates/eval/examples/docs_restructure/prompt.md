I need to refactor the multi-agent configuration system in our Arena-Baselines repository. The current policy_assignment parameter (self_play, independent) is too coarse. I want to replace it with a more flexible set of parameters to better support advanced training schemes like population-based training (PBT) and sophisticated self-play with historical opponents.

Specifically, I will introduce four new configuration parameters:

iterations_per_reload: Controls the frequency (in training iterations) at which policies are saved and potentially reloaded.
num_learning_policies: Explicitly defines how many agents use policies that are actively being trained (can be an integer or 'all').
selfplay_recent_prob: For non-learning agents (players), this determines the probability of loading the latest version of a learning policy versus loading a uniformly random historical version during reloads.
size_population: Specifies the number of distinct policy versions maintained for each learning agent, enabling PBT-style experiments.
To implement this, I will significantly modify train.py. This includes updating the argument parser, changing how experiment configurations are expanded (especially with grid_search), and implementing a new callback function (on_train_result). This callback will handle the periodic saving (using pickle) of learning policies to structured directories and the reloading of all policies (learning and playing) according to the new parameters (iterations_per_reload, selfplay_recent_prob, size_population). Playing policies will use deterministic actions.

I'll also reorganize the codebase by renaming arena/rllib_env.py to arena/arena.py and creating a new arena/utils.py file to house utility functions (like configuration helpers, ID generators, DeterministicCategorical) and constants.

Finally, I will update the example configuration files (Benchmark-2T1P-Discrete.yaml, Test-Pong.yaml) to remove policy_assignment and demonstrate the usage of the new parameters, including within grid_search.
