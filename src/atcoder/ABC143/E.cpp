//
// Created by inoue on 2019-10-19.
//

#include <iostream>
#include <vector>
#include <cmath>

using namespace std;

using ll = long long;

const ll INFL = 0x3f3f3f3f3f3f3f3f;

int main() {
    int N, M;
    ll L;

    cin >> N >> M >> L;

    vector<vector<ll>> edge(N, vector<ll>(N, INFL));
    vector<vector<ll>> dist(N, vector<ll>(N, INFL));

    for (int i=0; i<M; i++) {
        int a, b;
        ll c;
        cin >> a >> b >> c;
        if (c > L) continue;
        a--, b--;
        edge[a][b] = min(edge[a][b], c);
        edge[b][a] = min(edge[b][a], c);
        if (c <= L) {
            dist[a][b] = min(dist[a][b], c);
            dist[b][a] = min(dist[b][a], c);
        }
    }

    ll mod = 10000000000000;

    for (int k=0; k<N; k++) {
        for (int i=0; i<N; i++) {
            for (int j=0; j<N; j++) {
                if (edge[k][j] == INFL) continue;
                if (dist[i][k] == INFL) continue;
                ll cnt = dist[i][k] / mod;
                ll g = L - (dist[i][k] % mod);
                if (g >= edge[k][j]) {
                    g -= edge[k][j];
                }
                else {
                    cnt++;
                    g = L - edge[k][j];
                }
                if (i == 3 && k == 2 && j == 1) {
                    cout << cnt << " " << g << endl;
                }
                dist[i][j] = min(dist[i][j], cnt * mod + L - g);
            }
        }
    }

    int Q;
    cin >> Q;
    for (int q=0; q<Q; q++) {
        int s, t;
        cin >> s >> t;
        s--, t--;
        cout << (dist[s][t] == INFL ? -1 : dist[s][t] / mod) << endl;
    }
}