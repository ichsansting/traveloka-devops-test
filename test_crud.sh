get_all=$(curl -I -s --request GET http://localhost:8080/todos | awk 'FNR==1{print $2}')
[[ $get_all == 200 ]] && echo "get all todos OK" || echo "get all todos Not OK"

get_one=$(curl -I -s --request GET http://localhost:8080/todos/1 | awk 'FNR==1{print $2}')
[[ $get_one == 200 ]] && echo "get one todo OK" || echo "get one todo Not OK"

create_one=$(curl -I -s --request POST http://localhost:8080/todos/create?todo=new%20todo | awk 'FNR==1{print $2}')
[[ $create_one == 200 ]] && echo "create one todo OK" || echo "create one todo Not OK"

last_create=$(curl -s --request GET http://localhost:8080/todos/create/last-id)
echo "last create id is $last_create"

edit_one=$(curl -I -s --request PATCH http://localhost:8080/todos/1/edit?todo=edited%20todo | awk 'FNR==1{print $2}')
[[ $edit_one == 200 ]] && echo "edit one todo OK" || echo "edit one todo Not OK"

delete_one=$(curl -I -s --request DELETE http://localhost:8080/todos/$last_create/delete | awk 'FNR==1{print $2}')
[[ $delete_one == 200 ]] && echo "delete one todo OK" || echo "delete one todo Not OK"