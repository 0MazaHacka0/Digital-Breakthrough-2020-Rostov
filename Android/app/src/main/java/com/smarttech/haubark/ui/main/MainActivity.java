package com.smarttech.haubark.ui.main;

import android.os.Bundle;

import com.google.android.material.tabs.TabLayout;

import androidx.viewpager.widget.ViewPager;
import androidx.appcompat.app.AppCompatActivity;

import com.smarttech.haubark.R;

public class MainActivity extends AppCompatActivity {

    @Override
    protected void onCreate(Bundle savedInstanceState) {
        super.onCreate(savedInstanceState);
        setContentView(R.layout.activity_main);
        SectionsPagerAdapter sectionsPagerAdapter = new SectionsPagerAdapter(this, getSupportFragmentManager());
        ViewPager viewPager = findViewById(R.id.view_pager);
        viewPager.setAdapter(sectionsPagerAdapter);
        TabLayout tabs = findViewById(R.id.tabs);
        tabs.setupWithViewPager(viewPager);
        setupTabsIcons(tabs);
    }

    private void setupTabsIcons(TabLayout tabs) {
        tabs.getTabAt(0).setIcon(R.drawable.building);
        tabs.getTabAt(1).setIcon(R.drawable.ic_appeal);
        tabs.getTabAt(2).setIcon(R.drawable.ic_voting);
    }
}
