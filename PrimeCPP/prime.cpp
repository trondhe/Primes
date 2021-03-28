#include "prime.h"

PrimeSieve::PrimeSieve(int n) {
  sieveSize = n;
  bits_ = std::vector<bool>(n, true);
}

PrimeSieve::~PrimeSieve() {}

void PrimeSieve::runSieve() {
  int factor = 3;
  int q = sqrt(sieveSize);

  while (factor < q) {
    for (int num = factor; num < sieveSize; num++) {
      if (GetBit(num)) {
        factor = num;
        break;
      }
    }
    for (int num = factor * 3; num < sieveSize; num += factor * 2) {
      ClearBit(num);
    }

    factor += 2;
  }
}

void PrimeSieve::printResults(bool showResults, double duration, int passes) {
  if (showResults) {
    printf("2, ");
  }

  int count = 1;
  for (int num = 3; num <= sieveSize; num++) {
    if (GetBit(num)) {
      if (showResults) {
        printf("%d, ", num);
      }
      count++;
    }
  }

  if (showResults) {
    printf("\n");
  }

  printf("Passes: %d, Time: %lf, Avg: %lf, Limit: %d, Count: %d, Valid: %d\n",
         passes, duration, duration / passes, sieveSize, count,
         validateResults());
}

int PrimeSieve::countPrimes() {
  int count = 0;
  for (int i = 0; i < sieveSize; i++) {
    if (GetBit(i)) {
      count++;
    }
  }
  return count;
}

bool PrimeSieve::validateResults() {
  if (myDict.end() == myDict.find(sieveSize)) {
    return false;
  }
  return myDict.find(sieveSize)->second == countPrimes();
}

bool PrimeSieve::GetBit(uint32_t index) {
  if (index % 2 == 0) {
    return false;
  }
  return bits_[index];
}

void PrimeSieve::ClearBit(uint32_t index) {
  if (index % 2 == 0) {
    printf("You're setting even bits, which is sub-optimal.\n");
    return;
  }
  bits_[index] = false;
}
