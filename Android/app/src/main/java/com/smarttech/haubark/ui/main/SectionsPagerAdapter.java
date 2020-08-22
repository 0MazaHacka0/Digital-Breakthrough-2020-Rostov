package com.smarttech.haubark.ui.main;

import android.content.Context;

import androidx.annotation.Nullable;
import androidx.annotation.StringRes;
import androidx.fragment.app.Fragment;
import androidx.fragment.app.FragmentManager;
import androidx.fragment.app.FragmentPagerAdapter;

import com.smarttech.haubark.R;
import com.smarttech.haubark.ui.main.tabs.AppealFragment;
import com.smarttech.haubark.ui.main.tabs.InformationFragment;
import com.smarttech.haubark.ui.main.tabs.VoteListFragment;

/**
 * A [FragmentPagerAdapter] that returns a fragment corresponding to
 * one of the sections/tabs/pages.
 */
public class SectionsPagerAdapter extends FragmentPagerAdapter {

    @StringRes
    private static final int[] TAB_TITLES = new int[]{R.string.tab_text_1, R.string.tab_text_2};
//    private static final int[] TAB_ICONS = new int[]{R.raw.building, R.raw.building};

    private final Context mContext;

    public SectionsPagerAdapter(Context context, FragmentManager fm) {
        super(fm);
        mContext = context;
    }

    @Override
    public Fragment getItem(int position) {
        switch (position) {
            case 0:
                return InformationFragment.newInstance();
            case 1:
                return AppealFragment.newInstance();
            case 2:
                return VoteListFragment.newInstance();
        }
        return InformationFragment.newInstance();
    }


    @Nullable
    @Override
    public CharSequence getPageTitle(int position) {
        return null;
    }

    @Override
    public int getCount() {
        // Show 2 total pages.
        return 3;
    }
}