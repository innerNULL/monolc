#include <cuda_runtime.h>

__global__ void leaky_relu_kernel(const float* input, float* output, int N) {
    constexpr float alpha = 0.01;
    const int block_idx = blockIdx.x;
    const int block_dim = blockDim.x;
    const int thread_idx = threadIdx.x;
    const int index = block_idx * block_dim + thread_idx;
    if (index < N) {
        if (input[index] < 0) {
            output[index] = input[index] * alpha;
        } else {
            output[index] = input[index];
        }
    }
    return;
}

// input, output are device pointers (i.e. pointers to memory on the GPU)
extern "C" void solve(const float* input, float* output, int N) {
    int threadsPerBlock = 256;
    int blocksPerGrid = (N + threadsPerBlock - 1) / threadsPerBlock;
    
    leaky_relu_kernel<<<blocksPerGrid, threadsPerBlock>>>(input, output, N);
    cudaDeviceSynchronize();
}
