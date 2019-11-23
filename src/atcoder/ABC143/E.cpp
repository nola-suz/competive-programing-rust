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

    for (int k=0; k<N; k++) {
        for (int i=0; i<N; i++) {
            for (int j=0; j<N; j++) {
                dist[i][j] = min(dist[i][j], dist[i][k] + dist[k][j]);
            }
        }
    }

    vector<vector<ll>> dist2(N, vector<ll>(N, INFL));

    for(int i=0; i<N; i++) {
        for(int j=0; j<N; j++) {
            dist2[i][j] = (dist[i][j] <= L ? 1 : INFL);
        }
    }

    for (int k=0; k<N; k++) {
        for (int i=0; i<N; i++) {
            for (int j=0; j<N; j++) {
                dist2[i][j] = min(dist2[i][j], dist2[i][k] + dist2[k][j]);
            }
        }
    }

    int Q;
    cin >> Q;
    for (int q=0; q<Q; q++) {
        int s, t;
        cin >> s >> t;
        s--, t--;
        cout << (dist2[s][t] >= INFL ? -1 : dist2[s][t] - 1) << endl;
    }
}