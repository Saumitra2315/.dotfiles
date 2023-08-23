#include <bits/stdc++.h>
using namespace std;

// Macros for commonly used constants
#define INF 1e9
#define MOD 1000000007
#define endl '\n'

// Shortcuts for commonly used types
typedef long long ll;
typedef pair<int, int> pii;
typedef vector<int> vi;
typedef vector<vi> vvi;

//sort function
int sorts(int arr[], int n){
 int i, key, j;
    for (i = 1; i < n; i++) {
        key = arr[i];
        j = i - 1;
 
        
        while (j >= 0 && arr[j] > key) {
            arr[j + 1] = arr[j];
            j = j - 1;
        }
        arr[j + 1] = key;
    }
}

// Main function
int main() {
    ios_base::sync_with_stdio(false);
    cin.tie(NULL);
    //test cases
    int n;
    cin << n;
        
    // Your code goes here

    return 0;
}
