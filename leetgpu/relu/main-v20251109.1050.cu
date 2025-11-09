#include <cuda_runtime.h>

__global__ void relu_kernel(const float* input, float* output, int N) {
    const int block_idx = blockIdx.x;
    const int block_dim = blockDim.x;
    const int thread_idx = threadIdx.x;
    const int idx = block_idx * block_dim + thread_idx;
    if (idx >= N) {
        return;
    }
    if (input[idx] < 0) {
        output[idx] = 0;
    } else {
        output[idx] = input[idx];
    }
    return;
}

// input, output are device pointers (i.e. pointers to memory on the GPU)
extern "C" void solve(const float* input, float* output, int N) {
    int threadsPerBlock = 256;
    int blocksPerGrid = (N + threadsPerBlock - 1) / threadsPerBlock;

    relu_kernel<<<blocksPerGrid, threadsPerBlock>>>(input, output, N);
    cudaDeviceSynchronize();
}

