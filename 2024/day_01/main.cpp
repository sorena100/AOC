#include <iostream>
#include <fstream>
#include <string>
#include <vector>

void merge(std::vector<int>& arr, int left, int mid, int right);
void merge_sort(std::vector<int>& arr, int left, int right);

int main() {
    auto input_file_path = "input.txt";
    std::ifstream file(input_file_path);
    std::string line;
    std::vector<int> first_list;
    std::vector<int> second_list;
    int count = 0;
    while (std::getline(file, line)) {
        int space_index = line.find(' ');
        int first_number = std::stoi(line.substr(0, space_index));
        int second_number = std::stoi(line.substr(space_index + 1));
        first_list.push_back(first_number);
        second_list.push_back(second_number);
        count++;
    }

    merge_sort(first_list, 0, count - 1);
    merge_sort(second_list, 0, count - 1);

    int distance_all = 0;
    for (auto i = 0; i < count; i++) {
        distance_all += std::abs(first_list[i] - second_list[i]);
    }

    std::cout << "first part: " << distance_all << std::endl;

    int result = 0;
    for (auto i = 0; i < count; i++) {
        for (auto j = 0; j < count; j++) {
            if (first_list[i] == second_list[j]) {
                result += first_list[i];
            }
        }
    }

    std::cout << "second part: " << result << std::endl;

    return 0;
}

void merge(std::vector<int>& arr, int left,
                     int mid, int right)
{
    int n1 = mid - left + 1;
    int n2 = right - mid;

    // Create temp vectors
    std::vector<int> L(n1), R(n2);

    // Copy data to temp vectors L[] and R[]
    for (int i = 0; i < n1; i++)
        L[i] = arr[left + i];
    for (int j = 0; j < n2; j++)
        R[j] = arr[mid + 1 + j];

    int i = 0, j = 0;
    int k = left;

    // Merge the temp vectors back
    // into arr[left..right]
    while (i < n1 && j < n2) {
        if (L[i] <= R[j]) {
            arr[k] = L[i];
            i++;
        }
        else {
            arr[k] = R[j];
            j++;
        }
        k++;
    }

    // Copy the remaining elements of L[],
    // if there are any
    while (i < n1) {
        arr[k] = L[i];
        i++;
        k++;
    }

    // Copy the remaining elements of R[],
    // if there are any
    while (j < n2) {
        arr[k] = R[j];
        j++;
        k++;
    }
}

// begin is for left index and end is right index
// of the sub-array of arr to be sorted
void merge_sort(std::vector<int>& arr, int left, int right)
{
    if (left >= right)
        return;

    int mid = left + (right - left) / 2;
    merge_sort(arr, left, mid);
    merge_sort(arr, mid + 1, right);
    merge(arr, left, mid, right);
}

