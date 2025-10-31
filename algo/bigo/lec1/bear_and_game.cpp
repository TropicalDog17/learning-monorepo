#include<iostream>
using namespace std;
int main() {
    int next_bored = 15;
    int n;
    cin >> n;
    int interesting;
    for (int i = 0; i < n; i++) {
        cin >> interesting;

        // If next interesting is beyond the window, Bear Limak will stop watch at next_bored
        if (interesting > next_bored) {
            cout << next_bored;
            return 0;
        }

        // else, extend the next_bored
        next_bored = interesting + 15;
    }
    cout << min(next_bored, 90);
    return 0;
}