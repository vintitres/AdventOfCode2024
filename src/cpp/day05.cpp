#include <iostream>
#include <unordered_map>
#include <unordered_set>
#include <sstream>
#include <queue>

using namespace std;

vector<int> read(string const& line) {
    stringstream ss(line);
    vector<int> nums;
    string ns;
    while (getline(ss, ns, ',')) {
        int n;
        stringstream nss(ns);
        nss >> n;
        nums.push_back(n);
    }
    return nums;
}

int check(string const& line, unordered_map<int, unordered_set<int>> const& rules) {
    auto nums = read(line);
    unordered_set<int> seen;
    for (int n : nums) {
        seen.insert(n);
    }
    for (int n : nums) {
        seen.erase(n);
        auto rule_it = rules.find(n);
        if (rule_it != rules.end()) {
            for (int req : rule_it->second) {
                if (seen.find(req) != seen.end()) {
                    return 0;
                }
            }
        }
    }
    return nums[nums.size() / 2];
}

int fix(string const& line, unordered_map<int, unordered_set<int>> const& rules) {
    if (check(line, rules) != 0) {
        return 0;
    }
    auto nums = read(line);
    unordered_set<int> seen;
    deque<int> q;
    for (int n : nums) {
        seen.insert(n);
        q.push_back(n);
    }
    vector<int> new_nums;
    unordered_set<int> done;
    while (!q.empty()) {
        int n = q.front();
        if (done.find(n) != done.end()) {
            q.pop_front();
            continue;
        }
        auto rule_it = rules.find(n);
        if (rule_it != rules.end()) {
            bool any_req_added = false;
            for (int req : rule_it->second) {
                if (seen.find(req) != seen.end()) {
                    q.push_front(req);
                    any_req_added = true;
                }
            }
            if (any_req_added) {
                continue;
            }
        }
        seen.erase(n);
        q.pop_front();
        new_nums.push_back(n);
        done.insert(n);
    }

    return new_nums[new_nums.size() / 2];

}

int main() {
    string line = "s";
    unordered_map<int, unordered_set<int>> rules;
    while (line != "") {
        getline(cin, line);
        std::stringstream ss;
        ss << line;
        int n1, n2;
        char c;
        ss >> n1 >> c >> n2;
        auto it = rules.find(n2);
        if (it == rules.end()) {
            rules[n2] = unordered_set<int>();
            it = rules.find(n2);
        }
        (*it).second.insert(n1);
    }
    int sum1 = 0, sum2 = 0;
    while (cin >> line) {
        sum1 += check(line, rules);
        sum2 += fix(line, rules);
    }
    cout << sum1 << endl;
    cout << sum2 << endl;
    return 0;
}
