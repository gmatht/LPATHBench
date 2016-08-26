LPATHBench
==========

Benchmarks of the longest path problem in various languages

I added new benchmarks in the `rust/` directory. To run, type `cd rust; bash vs_c.sh`. This was to test the claim that ["it would not take long for even a Rust beginner to reimplement something equivalent"](https://news.ycombinator.com/item?id=11878759) to an given optimised C program. The big difference was that the C program unrolled the innermost loop. Unwinding the loop in rust as well gave similar performance. On my machine I get e.g.:

    model name	: Intel(R) Core(TM) i7-3770 CPU @ 3.40GHz
                           rustc 1.11.0 (9b21dcd6a 2016-08-15): 8981 LANGUAGE Rust 476
                                                       gcc -O2: 8981 LANGUAGE C-fast 657
                                                       gcc -O3: 8981 LANGUAGE C-fast 641
                                                     clang -O2: 8981 LANGUAGE C-fast 483
                                                     clang -O3: 8981 LANGUAGE C-fast 478
    clang -O3 -march=native -mtune=native -fomit-frame-pointer: 8981 LANGUAGE C-fast 480
                           rustc 1.11.0 (9b21dcd6a 2016-08-15): 8981 LANGUAGE Rust 467

The usual benchmarks are still available (see below)
--

`sh runbench.sh 8981 x86 x86html` to run the benchmark locally.
`sh runArmBench.sh 8981` to run the benchmark on an ARM device (edit the script to set the ssh and path settings)
`sh resdiff.sh x86 arm > diffgraph.html` to create the diff table
`python makeblog.py` to make the blog

Where 8981 is the distance of the longest path in the graph

If you want to make a new graph:

/mkgraph -places=NUM_NODES -worldsize=WORLD_SIZE, where NUM_NODES is the number of nodes in the graph, and WORLD_SIZE is the maximum distance between nodes. Each node has at least one connection, to the next node, and will on average have NUM_NODES/2 connections. Graphs are directed; a path of length N from Node1 to Node2 doesn't imply a similar path from Node2 to Node1.
