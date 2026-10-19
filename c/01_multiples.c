// n33l4m1n
// Project Euler
// September 29th, 2017
// ==========================================================
// Multiples of 3 and 5
// If we list all the natural numbers below 10 that
// are multiples of 3 or 5, we get 3, 5, 6 and 9.
// The sum of these multiples is 23.
// Find the sum of all the multiples of 3 or 5 below 1000.
// ==========================================================

#include <stdio.h>

int multiples(int n);

int main(void)
{
  int n = 1000; // 1000
  printf("%d\n", multiples(n));
  return 0;
}

int multiples(int n)
{
  int i;
  int sum = 0;
  
  for(i = 0; i < n; i++)
  {
    if(i % 3 == 0 || i % 5 == 0)
      sum = sum + i;
  }
    return sum;
}

// Congratulations, the answer you gave to problem 1 is correct.
// You are the 691843rd person to have solved this problem.
// 38ED0
