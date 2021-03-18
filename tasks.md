### Repository Clone

* **git-odb**
  * [x] all docs, sans examples
  * [x] Rename pack data/pack index `Kind` to `Version` or similar, because that's what it really is.
* **git-object** refactor
  * [x] split `Id` and everything hash related into `git-id`
  * [x] use `git-id` inside of `git-features`, remove cycle
* **git-config**
  * [x] Thanks to a generous contribution it's mostly done and well on the way
  * [ ] Push it towards 1.0
  * [ ] `Config` type which integrates multiple files into one interface, much like a *multi* version of `File`
  * [x] Make `gix organize` use `git-config` on single files (the repository configuration)
* **git-ref**
  * [ ] create ref pointing to ID
      * _assure to keep the path towards symbolic refs open, and allow specifying if these should be followed or not_
* **git-index**
  * [ ] Create an index from tree
  * [ ] Checkout index to worktree
* **git-repository**
  * [ ] instance for a valid looking repository
    * [ ] support shallow repos/references
  * [ ] create-update refs as received from clone/git-receive-pack
* **gix clone**
  * [ ] try initializing repo on output path - if so, use that to learn about pack location and place new pack there, allow Repo to create refs somehow.
    * _probably this is done using the repository itself, which steers the whole process and injects it's own delegates_.
  * [ ] otherwise create the scaffolding needed for a new repository, probably based on `init` implementation
* **receive pack**
  * [ ] resolve thin pack with Bundle
* **git-repository**
  * [ ] clone from https remote

### FSCK an entire repository

* **multi-db** (incorporate object lookup for loose objects and packs)
  * [ ] single threaded
  * [ ] optional object cache
  * [ ] fs-check - verify all object content of a git repository
    
### gix organize

* [ ] Add journey test to cover case with non-bare repository. Try to only read `non-bare` git config files and see the journey test fail.

### Commit-Graph

* [x] A plumbing command to extract some value from the current implementation, maybe statistics, or verification
* [x] Application of the command above in a stress test

### Feature Flags

* [ ] configure the `small` feature set so that the flate2 backend is miniz-oxide instead of zlib-ng, allowing a 'pure' rust build. 
      This might mean that the `fast` feature contains zlib-ng.

* **Questions**
  * What to do with the ['extra-garbage'](https://github.com/Byron/gitoxide/blob/6f90beeb418480f9cd8bb7ae3b5db678b24103cb/git-commitgraph/src/file/init.rs#L248),
    some code is commented out.
