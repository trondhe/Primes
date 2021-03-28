#include <bitset>
#include <chrono>
#include <ctime>
#include <iostream>
#include <map>

#include "prime.h"

int main() {
  auto passes = 0;
  auto tStart = std::chrono::steady_clock::now();
  while (true) {
    PrimeSieve sieve(1000000);
    sieve.runSieve();
    passes++;
    bool stopCondition = std::chrono::duration_cast<std::chrono::seconds>(
                             std::chrono::steady_clock::now() - tStart)
                             .count() >= 10;
    if (stopCondition) {
      auto tD = std::chrono::steady_clock::now() - tStart;
      double duration =
          std::chrono::duration_cast<std::chrono::microseconds>(tD).count() /
          1000000.;
      sieve.printResults(false, duration, passes);
      break;
    }
  }
}
