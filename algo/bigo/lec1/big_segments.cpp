#include<iostream>
#include<vector>
using namespace std;
int main() {
    int n;
    cin >> n;
    vector<pair<long, long> > B(n);
    long far_left = 1000000000, far_right = 0;
    for (int i = 0; i < n; i++) {
        cin >> B.at(i).first;
        far_left = min(far_left, B.at(i).first);
        cin >> B.at(i).second;
        far_right = max(far_right, B.at(i).second);
    }
    for (int i = 0; i < n; i++){
        if(B.at(i).first == far_left && B.at(i).second == far_right){
            cout << i + 1;
            return 0;
        }
    }

    cout << "-1";
}