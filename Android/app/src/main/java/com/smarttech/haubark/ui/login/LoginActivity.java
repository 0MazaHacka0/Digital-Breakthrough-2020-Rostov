package com.smarttech.haubark.ui.login;

import androidx.appcompat.app.AppCompatActivity;

import android.content.Intent;
import android.os.Bundle;
import android.view.View;
import android.widget.EditText;

import com.smarttech.haubark.R;
import com.smarttech.haubark.api.api.Api;
import com.smarttech.haubark.api.model.LoginRequest;
import com.smarttech.haubark.ui.flat_select.FlatSelectActivity;

import retrofit2.Call;
import retrofit2.Callback;
import retrofit2.Response;

public class LoginActivity extends AppCompatActivity {

    @Override
    protected void onCreate(Bundle savedInstanceState) {
        super.onCreate(savedInstanceState);
        setContentView(R.layout.activity_login);

        findViewById(R.id.btn_login).setOnClickListener(new View.OnClickListener() {
            @Override
            public void onClick(View view) {
//                login();
                startActivity(new Intent(getBaseContext(), FlatSelectActivity.class));
            }
        });
    }

    private void login() {
        String login = ((EditText) findViewById(R.id.input_phone)).getText().toString();
        String password = ((EditText) findViewById(R.id.input_password)).getText().toString();

        Api.createRetrofitAdapter().login(new LoginRequest(login, password)).enqueue(new Callback<Void>() {
            @Override
            public void onResponse(Call<Void> call, Response<Void> response) {
                if (response.isSuccessful()) {
                    startActivity(new Intent(getBaseContext(), FlatSelectActivity.class));
                }
            }

            @Override
            public void onFailure(Call<Void> call, Throwable t) {

            }
        });
    }
}
