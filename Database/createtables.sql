INSERT INTO Asset(a_id, a_name, a_sysname, a_model, a_type, a_manufactorer, a_ipaddress, a_purchasedate, a_note) values (0, 'PC1','Lenovo','Thinkpad','Laptop','Lenovo','192.168.1.2',20/07/2017, 'N/A');

insert into Department(d_id, d_name) values (0, 'Finance');
insert into Employee(employee_id, fname, sname, email, employment_status, department, assets) values (0, 'John', 'Doe', 'john.doe@example.com', 'Full-time', 'Finance', '0');
