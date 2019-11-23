
#include <cstdlib>
#include <cmath>
#include <climits>
#include <cfloat>
#include <map>
#include <set>
#include <iostream>
#include <string>
#include <vector>
#include <algorithm>
#include <sstream>
#include <complex>
#include <stack>
#include <queue>
#include <cstdio>
#include <cstring>
#include <iterator>
#include <bitset>
#include <unordered_set>
#include <unordered_map>
#include <fstream>
#include <iomanip>
#include <cassert>
#include <utility>
#include <memory>
#include <functional>
#include <deque>
#include <cctype>
#include <ctime>
#include <numeric>
#include <list>
#include <iomanip>

#if __cplusplus >= 201103L
#include <array>
#include <tuple>
#include <initializer_list>
#include <forward_list>

#define cauto const auto&
#else

#endif

using namespace std;


typedef long long ll;
typedef pair<int,int> pii;
typedef pair<ll,ll> pll;

typedef vector<int> vint;
typedef vector<vector<int> > vvint;
typedef vector<long long> vll;
typedef vector<vector<long long> > vvll;

#define VV(T) vector<vector< T > >

template <class T>
void initvv(vector<vector<T> > &v, int a, int b, const T &t = T()){
    v.assign(a, vector<T>(b, t));
}

template <class F, class T>
void convert(const F &f, T &t){
    stringstream ss;
    ss << f;
    ss >> t;
}

#define GET_MACRO(_1, _2, _3, NAME, ...) NAME
#define _rep(i,n) _rep2((i),0,(n))
#define _rep2(i,a,b) for(int i=(a);i<(b);++i)
#define rep(...) GET_MACRO(__VA_ARGS__, _rep2, _rep)(__VA_ARGS__)
#define ALL(v) (v).begin(),(v).end()
#define PB push_back
#define fi first
#define se second
#define mkp make_pair
#define DEBUG
#ifdef DEBUG
#define dump(x)  cout << #x << " = " << (x) << endl;
#define debug(x) cout << #x << " = " << (x) << " (L" << __LINE__ << ")" << " " << __FILE__ << endl;
#else
#define dump(x)
#define debug(x)
#endif

#define MOD 1000000007LL
#define EPS 1e-8
#define INF 0x3f3f3f3f
#define INFL 0x3f3f3f3f3f3f3f3fLL
#define maxs(x,y) x=max(x,y)
#define mins(x,y) x=min(x,y)

void mainmain(){
    int N, M;
    cin >> N >> M;

    vvint vv(N);
    rep(i, M) {
        int s, t;
        cin >> s >> t;
        s--, t--;
        vv[s].PB(t);
    }

    vector<long double> cost_p(N, 0);

    for (int i=N-2; i>=0; i--) {
        long double p = 1. / vv[i].size();

        for (int x: vv[i]) {
            cost_p[i] += p * (cost_p[x] + 1);
        }
    }

    vector<long double> move_p(N, 0);

    move_p[0] = 1;

    rep(i, N-1) {
        long double p = 1. / vv[i].size();

        for (int x: vv[i]) {
            move_p[x] += p * move_p[i];
        }
    }

    long double minus = 0;

    rep(i, N-1) {
        if (vv[i].size() < 2) continue;

        long double t_cost = 0;
        for (int x: vv[i]) {
            maxs(t_cost, cost_p[x]);
        }

        long double p = 1. / (vv[i].size() - 1);

        long double t_sum = 0;
        for (int x: vv[i]) {
            t_sum += cost_p[x] + 1;
        }

        t_sum -= t_cost + 1;

        t_sum *= p;

//        cout << i << " " << cost_p[i] << " " << move_p[i] << " " << t_cost << " " << t_sum << " " << move_p[i] * (cost_p[i] - t_sum) << endl;

        maxs(minus, move_p[i] * (cost_p[i] - t_sum));
    }

    cout << cost_p[0] - minus << endl;
}


signed main() {
    ios_base::sync_with_stdio(false);
    cin.tie(0);
    cout<<fixed<<setprecision(20);
    mainmain();
}