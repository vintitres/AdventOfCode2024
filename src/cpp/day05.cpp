#include <iostream>
using namespace std;

int main() {
    int l, r;
    vector<int> left, right;
    while(cin >> l >> r) {
        left.push_back(l);
        right.push_back(r);
    }
    sort(left.begin(), left.end());
    sort(right.begin(), right.end());
    auto left_it = left.begin();
    auto right_it = right.begin();
    int s = 0;
    for (; left_it != left.end();) {
        s += abs(*left_it - *right_it);
        ++right_it;
        ++left_it;
    }
    cout << s << endl;
    return 0;
}
