#include <iostream>
#include <vector>
#include <chrono>

using namespace std;
using namespace std::chrono;

int main()
{
    const int size = 64000000;
    const int skip = 1;

    vector<int> arr(64000000, 100);

    high_resolution_clock::time_point t1 = high_resolution_clock::now();

    for (int i = 0; i < size; i += skip)
    {
        arr[i] *= 3;
    }

    high_resolution_clock::time_point t2 = high_resolution_clock::now();
    duration<double> time_span = duration_cast<duration<double> >(t2 - t1);

    cout << time_span.count() << endl;
}