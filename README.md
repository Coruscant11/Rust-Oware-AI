# rust-oware-ai
A Rust implementation of an Oware AI. Using minimax algorithm, alphabeta pruning and move sorting.

The purpose of this project is to compare the performances to a similar project written with C++.
I used crossbeam::scope for the multithreading aspect, and there is some unit tests for the board.
This Rust version is apparently the faster one.

## How to run :
```
cargo run --release
```

## Performances

#### First move speed with four threads in this Rust version :
```
cargo run --release
```
<img width="638" alt="Capture d’écran 2022-01-07 à 14 58 04" src="https://user-images.githubusercontent.com/1645347/148554283-5c8860f8-f90a-4ce7-8792-b9828e2cfe3a.png">

#### The same move but in the C++ version (g++ -std=c++20 -Ofast *.cpp;./a.out):
```
g++ -std=c++20 -Ofast *.cpp
```
<img width="638" alt="Capture d’écran 2022-01-07 à 14 58 06" src="https://user-images.githubusercontent.com/1645347/148554319-9c91cc49-b8ef-44d3-be8f-4b631b7f4e5a.png">

## Conclusion
I did my best to write exactly the same algorithm between the two versions. Both of them are available on my github, do not hesitate to tell me if i made a mistake somewhere, making the implementation not fair.
I'm a litte bit surprised to see the Rust version win over C++. That's great !
