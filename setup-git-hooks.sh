#!/bin/bash

# -mindepth 1 = find won't list the git-hooks directory itself
# cp {} .git/hooks = copy the hook to git's directory
# chmod +x .git/hooks/$(basename {}) = makes the newly copied hook executable
find git-hooks -mindepth 1 -exec bash -c 'cp {} .git/hooks && chmod +x .git/hooks/$(basename {})' \;
