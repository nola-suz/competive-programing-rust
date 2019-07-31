#include <iostream>
#include <map>
#include <vector>

using namespace std;

#define INF 0x3f3f3f3f
#define fi first
#define se second


int main() {
    int N;
    cin >> N;
    vector<int> A(N);
    for(int i=0; i<N; i++) cin >> A[i];

    map<int,int> ma;
    ma[-1] = 1;
    ma[INF] = 1;

    for (int a: A) {
        auto it = ma.lower_bound(a);
        it--;

//        cout << "it " << it->fi << " " << it->se << endl;

        if (it->fi == -1) {
            if (ma.count(a) == 0) ma[a] = 0;
            ma[a]++;
        }
        else {
            ma[it->fi]--;
            if (ma[it->fi] == 0) ma.erase(it);
            if (ma.count(a) == 0) ma[a] = 0;
            ma[a]++;
        }

//        cout << "map" << endl;
//        for (auto x: ma) {
//            cout << x.fi << " " << x.se << endl;
//        }
//        cout << endl;
    }

    int ans = 0;
    for (auto x: ma) {
        if (x.fi == -1 || x.fi == INF) continue;
        ans += x.se;
    }
    cout << ans << endl;
}
