#include <cuda_runtime.h>

__global__ void reverse_array(float* input, int N) {
    const int block_idx = blockIdx.x;
    const int block_dim = blockDim.x;
    const int thread_idx = threadIdx.x;
    const int index = block_idx * block_dim + thread_idx;
    const int mid_point = (N - N % 2) / 2;
    if (index < mid_point) {
        const int symmetry_idx = N - 1 - index;
        const float tmp = input[index];
        input[index] = input[symmetry_idx];
        input[symmetry_idx] = tmp;
    }
    return;
}

// input is device pointer
extern "C" void solve(float* input, int N) {
    int threadsPerBlock = 256;
    int blocksPerGrid = (N + threadsPerBlock - 1) / threadsPerBlock;

    reverse_array<<<blocksPerGrid, threadsPerBlock>>>(input, N);
    cudaDeviceSynchronize();
}
