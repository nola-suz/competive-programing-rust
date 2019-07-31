#include <iostream>
#include <vector>

using namespace std;

int main() {
    int k = 2;

    for (int n=1; n<20; n++) {
        vector<int> v(n);
        for (int i=0; i < n; i++) {
            v[i] = i;
        }
        int tmp = 0;
        do {
            int t = 0;
            for(int i=0; i<n; i++) {
                t += abs(i - v[i]);
            }
            if (t == k) tmp++;
        }while(next_permutation(v.begin(), v.end()));
        cout << n << " " << k << " " << tmp << endl;
    }
}