//
// Created by inoue on 2019-09-28.
//

#include <iostream>
#include <set>

using namespace std;


//bool is_prime[1000010];

bool is_prime2(long long a) {
    if (a <= 1) return false;
    for (long long i = 2; i * i <= a; i++) {
        if (a % i == 0) {
            return false;
        }
    }
    return true;
}


int main() {
    long long a, b;
    cin >> a >> b;

//    for(int i=0; i<1000010; i++) is_prime[i] = true;
//    is_prime[0] = is_prime[1] = false;

//    for(int i=2; i<1000010; i++) {
//        if (!is_prime[i]) continue;
//
//        for(long long j=2*i; j*j <= 1000010; j+=i) {
//            is_prime[j] = false;
//        }
//    }

    set<long long> se;

    for (long long i=2; i*i <= a; i++) {
        if (a % i == 0) {
            if (is_prime2(i)) {
                se.insert(i);
            }
            if (i * i != a && is_prime2(a/i)) {
                se.insert(a/i);
            }
        }
    }

    if (is_prime2(a)) se.insert(a);

    int ans = 1;

    for (long long i=2; i*i <= b; i++) {
        if (b % i == 0) {
            if (se.find(i) != se.end()) {
                ans++;
            }
            if (i*i != b && se.find(b/i) != se.end()) {
                ans++;
            }
        }
    }

    if (se.find(b) != se.end()) {
        ans++;
    }

    cout << ans << endl;
}