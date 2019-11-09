//
// Created by inoue on 2019-09-28.
//

#include <iostream>
#include <set>
#include <vector>

using namespace std;

vector<vector<int>> edge;
set<int> loop;
vector<int> visited;

bool get_loop(int x) {
    if (loop.find(x) != loop.end()) return true;
    if (visited[x]) return false;
    visited[x] = 1;
    loop.insert(x);

    for (int y: edge[x]) {
        if (get_loop(y)) return true;
    }

    loop.erase(x);
    return false;
}


int main() {
    int n, m;
    cin >> n >> m;

    edge = vector<vector<int>>(n);

    for (int i=0; i < m; i++) {
        int a, b;
        cin >> a >> b;
        a--, b--;

        edge[a].push_back(b);
    }

    visited = vector<int>(n, 0);

    for (int i=0; i<n; i++) {
        loop.clear();
        if (get_loop(i)) {
            cout << loop.size() << endl;
            for(int x: loop) {
                cout << x + 1 << endl;
            }
            return 0;
        }
    }
    cout << -1 << endl;
}