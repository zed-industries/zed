1. README.md Features Section Reorganization
The features section has been reorganized into two subsections ("Baselines" and "Games") with markdown tables added. The previous bullet points were replaced with more structured content including supported/benchmarked status indicators. A new "Visualization" section was added with TensorBoard and port forwarding instructions.
2. Content Relocation and File Restructuring
The Tennis game documentation and action space details were moved from README.md to a new games.md file. The README was cleaned up by removing commented-out content and consolidating documentation sections. YAML config files (Benchmark-2T1P-Discrete.yaml and Test-Pong.yaml) were modified to replace `selfplay_recent_prob` with `playing_policy_load_recent_prob` and adjust population size options.
3. train.py Refactoring
Significant changes to train.py including:
- Renamed `selfplay_recent_prob` parameter to `playing_policy_load_recent_prob`
- Simplified the nested grid search structure by removing unnecessary loops
- Improved policy loading logic with better checkpoint path handling
- Enhanced error handling and logging for policy saving/reloading
- Removed redundant code and improved code organization
- Added more descriptive console output during policy operations
