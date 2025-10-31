#include<iostream>
#include<vector>
using namespace std;

int main() {
    int n, k;
    cin >> n >> k;

    vector<int> freq(100, 0);
    string curr;
    for (int i = 0; i < n; i++){
        cin >> curr;
        freq[curr.length()] += 1;
    }

    string correct_pass;
    cin >> correct_pass;

    int best = 0, worst = 0;
    for (int i = 1; i <= 100; i++) {
        if (i != correct_pass.length()) {
            best += freq[i];
            worst += freq[i];
            continue;
        }

        // if reach here, then i is curr.length()
        best += 1;
        worst += freq[correct_pass.length()];
        break;
    }



    int best_blocked_time = ((best-1) / k) * 5 + best;
    int worst_blocked_time = ((worst-1) / k)*5 + worst;

    cout << best_blocked_time << " " << worst_blocked_time;
}