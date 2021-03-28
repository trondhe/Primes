#pragma once
#include <bitset>
#include <chrono>
#include <ctime>
#include <iostream>
#include <map>
#include <vector>

class PrimeSieve {
public:
  PrimeSieve(int n);
  ~PrimeSieve();

  void runSieve();
  void printResults(bool showResults, double duration, int passes);
  int countPrimes();

private:
  int sieveSize = 0;
  std::vector<bool> bits_;
  const std::map<const int, const int> myDict = {
      {10, 1},   // Historical data for validating our results - the number of
                 // primes
      {100, 25}, // to be found under some limit, such as 168 primes under 1000
      {1000, 168},
      {10000, 1229},
      {100000, 9592},
      {1000000, 78498},
      {10000000, 664579},
      {100000000, 5761455}};

  bool validateResults();
  bool GetBit(uint32_t index);
  void ClearBit(uint32_t index);
};
