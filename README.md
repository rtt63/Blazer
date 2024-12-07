# Blazer

Tool for detecting hash changes between builds

## Idea

Go thru every file in src/, calc it's hash and put it into cachefile.
Cachefile is json path -> hash

Cachefile "remembers" the last state of the file, so if it was updated, we need to update file hash too.

Later, while transforming and minifying files, we need to put corresponding hash instead of placeholder [hash] or [blazerhash] or smth.

That's how we gonna update hashes for client side hashing, and not create new hash on each new build.
