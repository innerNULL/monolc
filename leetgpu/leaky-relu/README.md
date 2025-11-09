## Problem
```
Leaky ReLU
Easy
Implement a program that performs the leaky ReLU activation function on a vector of floating-point numbers. The leaky ReLU function is defined as:

 
where 
 is a small positive constant (0.01 in this problem).

Implementation Requirements
External libraries are not permitted
The solve function signature must remain unchanged
The final result must be stored in vector output
Use 
 as the leaky coefficient
Example 1:
  Input:  x = [1.0, -2.0, 3.0, -4.0]
  Output: y = [1.0, -0.02, 3.0, -0.04]
Example 2:
  Input:  x = [-1.5, 0.0, 2.5, -3.0]
  Output: y = [-0.015, 0.0, 2.5, -0.03]
Constraints
1 ≤ N ≤ 100,000,000
-1000.0 ≤ input[i] ≤ 1000.0
```

## Link
https://leetgpu.com/challenges/leaky-relu
