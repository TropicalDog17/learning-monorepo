#include<iostream>

using namespace std;
int main() {
    string s ,t;
    cin >> s;
    cin >> t;
    string m;

    // let m to be clone of s first
    m = s;
    int str_len = s.length();
    for (int i = str_len - 1; i >= 0; i--) {
        if (m[i] != 'z') {
            m[i] += 1;
            break;
        } else{
            m[i] = 'a';
        }
    }
    if (m < t) {
        cout << m;
        return 0;
    }

    cout << "No such string";
}