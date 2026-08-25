#!/bin/bash

#----------------------------------------------------------------------------------------------------

echo "Clearing all data..."
curl -X DELETE http://127.0.0.1:3000/steps/delete_all && echo "Done"
  
#----------------------------------------------------------------------------------------------------

echo "Checking submit step count - user A"
resp=$(curl -X POST http://127.0.0.1:3000/steps/record \
            -s \
            -H 'Content-Type: application/json' \
            -d '{ "date_time": "2026-08-24T18:43:52Z",
                  "step_count": 4000,
                  "user_id": "3fa85f64-5717-4562-b3fc-2c963f66afa6"
                }')
echo $resp
exp='{"all_users_total_step_count":4000}'
[[ "$resp" == "$exp" ]] || { echo "Expected '$exp', got '$resp'"; exit 1; } && echo "Done"

#----------------------------------------------------------------------------------------------------

echo "Checking submit step count - user B"
resp=$(curl -X POST http://127.0.0.1:3000/steps/record \
            -s \
            -H 'Content-Type: application/json' \
            -d '{ "date_time": "2026-08-24T12:00:12Z",
                  "step_count": 3300,
                  "user_id": "0b2d53c0-9fed-11f1-b8b5-08bfb8034f7c"
                }')
echo $resp
exp='{"all_users_total_step_count":7300}'
[[ "$resp" == "$exp" ]] || { echo "Expected '$exp', got '$resp'"; exit 1; } && echo "Done"

#----------------------------------------------------------------------------------------------------

echo "Submitting more steps for user A"
resp=$(curl -X POST http://127.0.0.1:3000/steps/record \
            -s \
            -H 'Content-Type: application/json' \
            -d '{ "date_time": "2026-07-23T14:00:00Z",
                  "step_count": 2000,
                  "user_id": "3fa85f64-5717-4562-b3fc-2c963f66afa6"
                }')
echo $resp
exp='{"all_users_total_step_count":9300}'
[[ "$resp" == "$exp" ]] || { echo "Expected '$exp', got '$resp'"; exit 1; } && echo "Done"

#----------------------------------------------------------------------------------------------------

echo "Checking user A's step count"
resp=$(curl -s http://127.0.0.1:3000/steps/user-step-count?user_id=3fa85f64-5717-4562-b3fc-2c963f66afa6)
echo $resp
exp='{"step_count":6000}'
[[ "$resp" == "$exp" ]] || { echo "Expected '$exp', got '$resp'"; exit 1; } && echo "Done"

#----------------------------------------------------------------------------------------------------

echo "Checking user B's step count"
resp=$(curl -s http://127.0.0.1:3000/steps/user-step-count?user_id=0b2d53c0-9fed-11f1-b8b5-08bfb8034f7c)
echo $resp
exp='{"step_count":3300}'
[[ "$resp" == "$exp" ]] || { echo "Expected '$exp', got '$resp'"; exit 1; } && echo "Done"

#----------------------------------------------------------------------------------------------------

echo "Query all users by date range"
resp=$(curl -s 'http://127.0.0.1:3000/steps/all-users-step-count?start=2026-06-23T14%3A00%3A00Z&end=2026-08-01T14%3A00%3A00Z')
echo $resp
exp='{"all_users_total_step_count":2000}'
[[ "$resp" == "$exp" ]] || { echo "Expected '$exp', got '$resp'"; exit 1; } && echo "Done"

#----------------------------------------------------------------------------------------------------

echo "Query one user by date range"
resp=$(curl -s 'http://127.0.0.1:3000/steps/user-step-count?user_id=3fa85f64-5717-4562-b3fc-2c963f66afa6&start=2026-08-23T18%3A43%3A52Z&end=2026-08-25T18%3A43%3A52Z')
echo $resp
exp='{"step_count":4000}'
[[ "$resp" == "$exp" ]] || { echo "Expected '$exp', got '$resp'"; exit 1; } && echo "Done"

#----------------------------------------------------------------------------------------------------

echo "Success"