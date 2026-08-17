#!/usr/bin/env perl

#------------------------------------------------------------------------------
#
# This program turn all the tests under the src/ directory into test data files
# under the `data` directory.
#
#------------------------------------------------------------------------------

use strict; use warnings;
use FindBin;
use lib $FindBin::Bin;
use base 'YAMLTestSuite';
use Encode;
use File::Path qw(make_path);

my %map = (
    'name' => '===',
    'fail' => 'error',
    'yaml' => 'in.yaml',
    'tree' => 'test.event',
    'json' => 'in.json',
    'dump' => 'out.yaml',
    'emit' => 'emit.yaml',
    'toke' => 'lex.token',
);

die "'data' directory not empty" if glob('data/*');
# mkdir my $meta_dir = "data/meta";
mkdir my $name_dir = "data/name";
mkdir my $tags_dir = "data/tags";

main->new->run([@ARGV]);

sub make {
    my ($self) = @_;

    my ($id, $ID, $num, $data, $multi, $slug) =
        @$self{qw<id ID num data multi slug>};

    my $test_dir = "data/$id";
    mkdir $test_dir unless -d $test_dir;

    if ($multi) {
        $test_dir .= "/$num";
        mkdir $test_dir or die $test_dir;
    }

    for my $k (sort keys %map) {
        $_ = $data->{$k};
        if (defined $_) {
            if ($k eq 'name') {
                $_ .= "\n";
            }
            elsif ($k eq 'fail') {
                next unless $_;
                $_ = '';
            }
            elsif ($k eq 'tree') {
                s/^\s+//mg;
                s/\n*\z/\n/;
            }

            $_ = $self->unescape($_);

            open my $out, '>', "$test_dir/$map{$k}" or die;
            print $out encode_utf8($_);
            close $out;
        }
    }

    if ($num == 0) {
        # symlink $data->{name}, "$meta_dir/$id.label";
        symlink "../$id", "$name_dir/$slug";
    }

    for my $tag (split /\s+/, $data->{tags}) {
        mkdir "$tags_dir/$tag";
        if ($multi) {
            mkdir "$tags_dir/$tag/$id";
            symlink "../../../$id/$num", "$tags_dir/$tag/$id/$num";
        }
        else {
            symlink "../../$id", "$tags_dir/$tag/$id";
        }
    }
}

sub final {
    my ($self) = @_;
    warn "Wrote $self->{make} data tests.\n";
}
