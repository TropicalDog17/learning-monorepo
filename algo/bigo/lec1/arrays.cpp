#include<iostream>
#include <vector>

using namespace std;

int main() {
    int nA, nB, k, m;
    cin >> nA >> nB;
    cin >> k >> m;
    vector<int> A(nA, 0);
    vector<int> B(nB, 0);
    for(int i = 0; i < nA; i++){
        cin >> A[i];
    }
    for (int i = 0; i < nB; i++){
        cin >> B[i];
    }

    if (A[k-1] < B[nB - m]) {
        cout << "YES";
    } else{
        cout << "NO";
    }

}