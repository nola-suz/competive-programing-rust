//
// Created by inoue on 2019-09-28.
//

#include <iostream>
#include <vector>

using namespace std;

#define INF 0x3f3f3f3f

int main() {
    int n, m;
    cin >> n >> m;

    vector<int> dp(1<<13, INF);


    for (int i=0; i<m; i++) {
        int a, b;
        cin >> a >> b;
        int c = 0;
        for (int j=0; j<b; j++) {
            int t;
            cin >> t;
            t--;
            c += 1 << t;
        }

        dp[c] = min(dp[c], a);
    }

    for (int i=0; i<(1<<12); i++) {
        for (int j=0; j<i; j++) {
            dp[i|j] = min(dp[i|j], dp[i] + dp[j]);
        }
    }

    if (dp[(1<<n) - 1] == INF) {
        cout << -1 << endl;
    } else {
        cout << dp[(1<<n) - 1] << endl;
    }
}