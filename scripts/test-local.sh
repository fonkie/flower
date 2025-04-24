#!/bin/bash

# test-flower-api.sh
# A comprehensive test script for the flower API
# This script runs curl commands to test all the documented endpoints

# Color codes for better readability
GREEN='\033[0;32m'
RED='\033[0;31m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Configuration
API_HOST="http://localhost:10086"
CONTENT_TYPE="accept: application/json"
TOTAL_TESTS=0
PASSED_TESTS=0
FAILED_TESTS=0

# URL encoding function for special characters including Chinese
urlencode() {
  local string="$1"
  local length="${#string}"
  local encoded=""
  local pos c o
  
  for (( pos=0; pos<length; pos++ )); do
    c="${string:$pos:1}"
    case "$c" in
      [-_.~a-zA-Z0-9]) # RFC 3986 unreserved characters
        encoded+="$c"
        ;;
      *)
        printf -v o '%%%02X' "'$c"
        encoded+="$o"
        ;;
    esac
  done
  echo "${encoded}"
}

# Define and encode the Chinese search term
CHINESE_TERM="干细胞"
ENCODED_CHINESE_TERM=$(urlencode "$CHINESE_TERM")

# Helper function to run a test
run_test() {
    local test_name="$1"
    local curl_command="$2"
    
    echo -e "\n${YELLOW}Running test: ${test_name}${NC}"
    echo -e "${BLUE}Command: ${curl_command}${NC}"
    
    # Execute the command with extra options to capture HTTP status and output
    # -s: silent, -w: write-out format, -o: output to file
    local http_status
    local temp_file=$(mktemp)
    
    # Add -s flag if not already included
    if [[ "$curl_command" != *"-s "* ]]; then
        curl_command="${curl_command/curl/curl -s}"
    fi
    
    # Execute curl with status capture
    http_status=$(eval "${curl_command} -w '%{http_code}' -o ${temp_file}")
    response=$(cat ${temp_file})
    rm ${temp_file}
    
    # Update test counters
    TOTAL_TESTS=$((TOTAL_TESTS + 1))
    
    # Check if the API call was successful (2xx status code)
    if [[ $http_status -ge 200 && $http_status -lt 300 ]]; then
        echo -e "${GREEN}✓ Test passed (HTTP status: ${http_status})${NC}"
        PASSED_TESTS=$((PASSED_TESTS + 1))
        
        # Check if response contains an error message despite status code
        if echo "${response}" | grep -q '"detail"' && echo "${response}" | grep -q 'error'; then
            echo -e "${YELLOW}⚠ Warning: Success status code but error in response body${NC}"
            echo -e "${YELLOW}Error details:${NC}"
            echo "${response}" | grep -o '"detail":"[^"]*"' | sed 's/"detail":"//g' | sed 's/"//g'
            PASSED_TESTS=$((PASSED_TESTS - 1))
            FAILED_TESTS=$((FAILED_TESTS + 1))
        else
            # Print a sample of the response (first 300 characters)
            echo -e "${BLUE}Response (truncated):${NC}"
            echo "${response}" | head -c 300
            echo "..."
        fi
    else
        echo -e "${RED}✗ Test failed (HTTP status: ${http_status})${NC}"
        echo -e "${RED}Error response:${NC}"
        echo "${response}"
        FAILED_TESTS=$((FAILED_TESTS + 1))
    fi
}

echo -e "${YELLOW}===================================${NC}"
echo -e "${YELLOW}   FLOWER API TEST SUITE          ${NC}"
echo -e "${YELLOW}===================================${NC}"
echo -e "Testing API at: ${API_HOST}\n"

# ===== Basic Information Tests =====
echo -e "${YELLOW}===== Basic Information Tests =====${NC}"

# Test 1: Get API information
run_test "Get API Information" "curl -s -X GET \"${API_HOST}/\" -H \"${CONTENT_TYPE}\""

# ===== Posts Tests =====
echo -e "\n${YELLOW}===== Posts Tests =====${NC}"

# Test 2: Get all published posts (with default pagination)
run_test "Get All Posts" "curl -s -X GET \"${API_HOST}/api/v1/posts\" -H \"${CONTENT_TYPE}\""

# Test 3: Get posts with pagination
run_test "Get Posts with Pagination" "curl -s -X GET \"${API_HOST}/api/v1/posts?page=2&page_size=5\" -H \"${CONTENT_TYPE}\""

# Test 4: Get a specific post
run_test "Get Specific Post" "curl -s -X GET \"${API_HOST}/api/v1/posts/1\" -H \"${CONTENT_TYPE}\""

# Test 5: Search posts containing "干细胞" (stem cell) - UPDATED with URL encoding
run_test "Search Posts" "curl -s -X GET \"${API_HOST}/api/v1/posts?search=${ENCODED_CHINESE_TERM}\" -H \"${CONTENT_TYPE}\""

# ===== Post Types Tests =====
echo -e "\n${YELLOW}===== Post Types Tests =====${NC}"

# Test 6: Get all post types
run_test "Get All Post Types" "curl -s -X GET \"${API_HOST}/api/v1/post-types\" -H \"${CONTENT_TYPE}\""

# Test 7: Get all pages
run_test "Get All Pages" "curl -s -X GET \"${API_HOST}/api/v1/post-types/page/posts\" -H \"${CONTENT_TYPE}\""

# Test 8: Get all products
run_test "Get All Products" "curl -s -X GET \"${API_HOST}/api/v1/post-types/products/posts\" -H \"${CONTENT_TYPE}\""

# Test 9: Get all news articles
run_test "Get All News Articles" "curl -s -X GET \"${API_HOST}/api/v1/post-types/news/posts\" -H \"${CONTENT_TYPE}\""

# Test 10: Get all customer testimonials
run_test "Get All Customer Testimonials" "curl -s -X GET \"${API_HOST}/api/v1/post-types/customer/posts\" -H \"${CONTENT_TYPE}\""

# ===== Post Metadata Tests =====
echo -e "\n${YELLOW}===== Post Metadata Tests =====${NC}"

# Test 11: Get metadata for a post
run_test "Get Post Metadata" "curl -s -X GET \"${API_HOST}/api/v1/posts/1/meta\" -H \"${CONTENT_TYPE}\""

# ===== Categories Tests =====
echo -e "\n${YELLOW}===== Categories Tests =====${NC}"

# Test 12: Get all categories
run_test "Get All Categories" "curl -s -X GET \"${API_HOST}/api/v1/categories\" -H \"${CONTENT_TYPE}\""

# Test 13: Get posts in a specific category
run_test "Get Posts in Category" "curl -s -X GET \"${API_HOST}/api/v1/categories/2/posts\" -H \"${CONTENT_TYPE}\""

# ===== Advanced Queries Tests =====
echo -e "\n${YELLOW}===== Advanced Queries Tests =====${NC}"

# Test 14: Get draft posts
run_test "Get Draft Posts" "curl -s -X GET \"${API_HOST}/api/v1/posts?post_status=draft\" -H \"${CONTENT_TYPE}\""

# Test 15: Get posts by a specific author
run_test "Get Posts by Author" "curl -s -X GET \"${API_HOST}/api/v1/posts?author_id=1\" -H \"${CONTENT_TYPE}\""

# Test 16: Get posts with multiple filters
run_test "Get Posts with Multiple Filters" "curl -s -X GET \"${API_HOST}/api/v1/posts?post_type=post&post_status=publish&author_id=1&search=wordpress\" -H \"${CONTENT_TYPE}\""

# ===== Sanyuan Specific Tests =====
echo -e "\n${YELLOW}===== Sanyuan Specific Tests =====${NC}"

# Test 17: View the "走进三源长生" (About Sanyuan) page
run_test "Get About Sanyuan Page" "curl -s -X GET \"${API_HOST}/api/v1/posts/11\" -H \"${CONTENT_TYPE}\""

# Test 18: Get all products from the Sanyuan website
run_test "Get All Sanyuan Products" "curl -s -X GET \"${API_HOST}/api/v1/post-types/products/posts?page_size=100\" -H \"${CONTENT_TYPE}\""

# Test 19: Search for news articles containing "干细胞" (stem cell) - UPDATED with URL encoding
run_test "Search News Articles with 干细胞" "curl -s -X GET \"${API_HOST}/api/v1/post-types/news/posts?search=${ENCODED_CHINESE_TERM}\" -H \"${CONTENT_TYPE}\""

# Test 20: Get customer testimonials
run_test "Get Customer Testimonials" "curl -s -X GET \"${API_HOST}/api/v1/post-types/customer/posts\" -H \"${CONTENT_TYPE}\""

# ===== Summary =====
echo -e "\n${YELLOW}===== Test Suite Complete =====${NC}"
echo -e "Completed all tests for the flower API"
echo -e "Results:"
echo -e "  ${BLUE}Total tests:  ${TOTAL_TESTS}${NC}"
echo -e "  ${GREEN}Tests passed: ${PASSED_TESTS}${NC}"
echo -e "  ${RED}Tests failed: ${FAILED_TESTS}${NC}"
echo -e "\nRun with verbose output to see full responses:"
echo -e "${BLUE}  ./${0##*/} --verbose${NC}"

# Add an option for verbose output
if [[ "$1" == "--verbose" ]]; then
    echo -e "\n${YELLOW}===== Running Verbose Test for Documentation =====${NC}"
    echo -e "Below is a full test suite with complete output that can be used for documentation:"
    
    echo '```bash'
    # API Information
    echo "# Get API information"
    echo "curl -X GET \"${API_HOST}/\" -H \"${CONTENT_TYPE}\""
    curl -s -X GET "${API_HOST}/" -H "${CONTENT_TYPE}" | jq . 2>/dev/null || echo "Error: Failed to parse JSON response"
    echo ""
    
    # Get posts with different parameters
    echo "# Get all published posts"
    echo "curl -X GET \"${API_HOST}/api/v1/posts\" -H \"${CONTENT_TYPE}\""
    curl -s -X GET "${API_HOST}/api/v1/posts" -H "${CONTENT_TYPE}" | jq . 2>/dev/null || echo "Error: Failed to parse JSON response"
    echo ""
    
    echo "# Get a specific post by ID"
    echo "curl -X GET \"${API_HOST}/api/v1/posts/1\" -H \"${CONTENT_TYPE}\""
    curl -s -X GET "${API_HOST}/api/v1/posts/1" -H "${CONTENT_TYPE}" | jq . 2>/dev/null || echo "Error: Failed to parse JSON response"
    echo ""
    
    # Also update the verbose examples with encoded characters
    echo "# Search posts containing stem cell (干细胞)"
    echo "curl -X GET \"${API_HOST}/api/v1/posts?search=${ENCODED_CHINESE_TERM}\" -H \"${CONTENT_TYPE}\""
    curl -s -X GET "${API_HOST}/api/v1/posts?search=${ENCODED_CHINESE_TERM}" -H "${CONTENT_TYPE}" | jq . 2>/dev/null || echo "Error: Failed to parse JSON response"
    echo ""
    
    # Add more examples as needed for documentation purposes
    echo '```'
fi

echo -e "\n${GREEN}Done!${NC}"