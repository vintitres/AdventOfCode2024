#include <iostream>
#include <unordered_map>
#include <unordered_set>
#include <sstream>

using namespace std;

int check(string const& line, unordered_map<int, unordered_set<int>> const& rules) {
    unordered_set<int> seen;
    stringstream ss(line);
    vector<int> nums;
    string ns;
    while (getline(ss, ns, ',')) {
        int n;
        stringstream nss(ns);
        nss >> n;
        nums.push_back(n);
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
    int sum = 0;
    while (cin >> line) {
        sum += check(line, rules);
    }
    cout << sum << endl;
    return 0;
}
